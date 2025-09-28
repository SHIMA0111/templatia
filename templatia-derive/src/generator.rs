use std::collections::{HashMap, HashSet};
use quote::{quote};
use crate::parser::TemplateSegments;

pub(crate) fn generate_str_parser(
    struct_name: &syn::Ident,
    all_fields: &[syn::Field],
    placeholder_names: &HashSet<String>,
    segments: &[TemplateSegments]
) -> proc_macro2::TokenStream {
    // Validate placeholders exist as fields
    let all_field_names = all_fields
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|id| id.to_string()))
        .collect::<HashSet<_>>();

    for name in placeholder_names {
        if !all_field_names.contains(name) {
            let error = syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("{} has no field named \"{}\"", struct_name.to_string(), name),
            );
            return error.to_compile_error().into();
        }
    }

    // Build ordered list of placeholders and their types
    let mut ordered_idents: Vec<syn::Ident> = Vec::new();
    let mut ordered_types: Vec<syn::Type> = Vec::new();

    for segment in segments.iter() {
        if let TemplateSegments::Placeholder(name) = segment {
            let name_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            if let Some(field) = all_fields.iter().find(|f| f.ident.as_ref() == Some(&name_ident)) {
                ordered_idents.push(name_ident);
                ordered_types.push(field.ty.clone());
            } else {
                let err = syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("{} has no field named \"{}\"", struct_name.to_string(), name),
                );
                return err.to_compile_error().into();
            }
        }
    }

    // Generate imperative parser code that reads from `s`
    // It walks the template segments, checks literals, and captures slices for placeholders.
    let mut capture_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut value_idents: Vec<syn::Ident> = Vec::new();

    let mut iter = segments.iter().peekable();
    while let Some(seg) = iter.next() {
        match seg {
            TemplateSegments::Literal(lit) => {
                let lit_str = *lit;
                capture_stmts.push(quote! {
                    if !s[idx..].starts_with(#lit_str) {
                        return Err(templatia::TemplateError::Parse(format!("Expected literal {:?} at position {}", #lit_str, idx)));
                    }
                    idx += #lit_str.len();
                });
            }
            TemplateSegments::Placeholder(name) => {
                let name_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                let ty = if let Some(pos) = ordered_idents.iter().position(|id| id == &name_ident) {
                    ordered_types[pos].clone()
                } else {
                    // Already validated above, unreachable
                    syn::parse_quote! { () }
                };

                // Determine the next literal (if any)
                let next_lit_opt = match iter.peek() {
                    Some(TemplateSegments::Literal(lit)) => Some(*lit),
                    _ => None,
                };

                let val_ident = syn::Ident::new(&format!("__val_{}", name), proc_macro2::Span::call_site());
                value_idents.push(name_ident.clone());

                let capture_code = if let Some(next_lit) = next_lit_opt {
                    quote! {
                        let end = s[idx..].find(#next_lit).ok_or_else(|| templatia::TemplateError::Parse(
                            format!("Expected delimiter {:?} for placeholder '{}' not found", #next_lit, stringify!(#name_ident))
                        ))?;
                        let slice = &s[idx..idx + end];
                        idx += end; // do not consume delimiter here; next literal arm will consume it
                        let #val_ident: #ty = slice.parse().map_err(|e| templatia::TemplateError::Parse(
                            format!("Failed to parse field \"{}\": {}", stringify!(#name_ident), e)
                        ))?;
                    }
                } else {
                    quote! {
                        let slice = &s[idx..];
                        idx = s.len();
                        let #val_ident: #ty = slice.parse().map_err(|e| templatia::TemplateError::Parse(
                            format!("Failed to parse field \"{}\": {}", stringify!(#name_ident), e)
                        ))?;
                    }
                };

                capture_stmts.push(capture_code);
            }
        }
    }

    // Now assign captured values to struct fields in the order of first appearance,
    // using the last seen value if a placeholder repeats.
    let mut seen: HashSet<String> = HashSet::new();
    let mut struct_field_inits: Vec<proc_macro2::TokenStream> = Vec::new();
    for ident in ordered_idents.iter() {
        let key = ident.to_string();
        if seen.insert(key.clone()) {
            let val_ident = syn::Ident::new(&format!("__val_{}", key), proc_macro2::Span::call_site());
            struct_field_inits.push(quote! { #ident: #val_ident });
        }
    }

    quote! {{
        let mut idx: usize = 0;
        #(#capture_stmts)*
        if idx != s.len() {
            return Err(templatia::TemplateError::Parse(format!("Unexpected trailing characters at position {}", idx)));
        }
        Ok(#struct_name { #(#struct_field_inits),* })
    }}}

fn generate_field_parser(
    field_name: &syn::Ident,
    field_type: &syn::Type,
    next_segment: Option<&TemplateSegments>
) -> proc_macro2::TokenStream {
    let next_literal = match next_segment {
        Some(TemplateSegments::Literal(lit)) => Some(lit),
        _ => None,
    };

    let value_extractor = if let Some(next_literal) = next_literal {
        quote! {
            chumsky::text::keyword(#next_literal)
                .not()
                .ignore_then(chumsky::prelude::any())
                .repeated()
                .to_slice()
        }
    } else {
        quote! {
            chumsky::prelude::any().repeated().to_slice()
        }
    };

    // CAUTION: In this generator, the try_map isn't called to the TokenStream; it calls the chumsky Object generated from to_slice().
    quote! {
        #value_extractor.try_map(|s: &str, span| {
            s.parse::<#field_type>()
                .map_err(|e| chumsky::error::Rich::<char>::custom(
                    span,
                    format!("Failed to parse field \"{}\": {}", stringify!(#field_name), e)
                ))
        })
    }
}

fn generate_tuple_pattern(
    seen_field_names: &mut HashMap<String, usize>,
    field_names: &Vec<syn::Ident>,
) -> proc_macro2::TokenStream {
    // If already seen, return true
    let mut key_generator = |key: &syn::Ident| -> syn::Ident {
        let res = seen_field_names
            .entry(key.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);

        if *res > 1 {
            // TODO: We should handle the duplicates template placeholder
            let new_key = format!("_{}_{}", key, res);
            syn::Ident::new(&new_key, proc_macro2::Span::call_site())
        } else {
            key.clone()
        }
    };

    if !field_names.is_empty() {
        let mut pattern_iter = field_names.iter();
        if field_names.len() > 1 {
            // SAFETY: In this branch, the condition is field_names.len() > 1, so the first, second must be success.
            let first = key_generator(pattern_iter.next().unwrap());
            let second = key_generator(pattern_iter.next().unwrap());

            let mut current_pattern = quote! { (#first, #second) };

            for next_field in pattern_iter {
                let next_field = key_generator(next_field);
                current_pattern = quote! { (#current_pattern, #next_field) };
            }
            current_pattern
        } else {
            // SAFETY: In this branch, the field_names is not empty and not len() > 1 so the len() must be 1.
            let first = pattern_iter.next().unwrap();
            quote! { #first }
        }
    } else {
        quote! { _ }
    }
}
