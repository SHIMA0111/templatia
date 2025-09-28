use std::collections::{HashMap, HashSet};
use chumsky::Parser;
use quote::{quote};
use crate::parser::TemplateSegments;

pub(crate) fn generate_str_parser(
    struct_name: &syn::Ident,
    all_fields: &[syn::Field],
    placeholder_names: &HashSet<String>,
    segments: &[TemplateSegments]
) -> proc_macro2::TokenStream {
    // Get the field name
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


    let mut parsers = segments.iter().peekable();
    let mut generated_full_parser = quote! { chumsky::prelude::empty() };
    let mut placeholder_count = 0;

    while let Some(segment) = parsers.next() {
        match segment {
            TemplateSegments::Literal(lit) => {
                if placeholder_count == 0 {
                    generated_full_parser = quote! { chumsky::prelude::just(#lit).ignored() }
                } else {
                    generated_full_parser = quote! { #generated_full_parser.then_ignore(chumsky::prelude::just(#lit)) }
                }
            },
            TemplateSegments::Placeholder(name) => {
                // Get the placeholder name on Ident
                let name_ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                let field = match all_fields
                    .iter()
                    // Check if the placeholder name contains the field name or not
                    .find(|f| f.ident.as_ref() == Some(&name_ident))
                {
                    Some(field) => field,
                    None => {
                        let err = syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("{} has no field named \"{}\"", struct_name.to_string(), name),
                        );
                        return err.to_compile_error().into()
                    }
                };

                let field_type = &field.ty;
                let field_parser = generate_field_parser(&name_ident, field_type, parsers.peek().cloned());

                if placeholder_count == 0 {
                    generated_full_parser = field_parser;
                } else {
                    generated_full_parser = quote! { #generated_full_parser.then(#field_parser) }
                }

                // Count of the placeholder
                placeholder_count += 1;
            }
        }
    }

    generated_full_parser = quote! { #generated_full_parser.then_ignore(chumsky::prelude::end()) };

    let field_names = segments
        .iter()
        .filter_map(|segment| match segment {
            TemplateSegments::Placeholder(name) => Some(syn::Ident::new(&name, proc_macro2::Span::call_site())),
            _ => None,
        }).collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut counter = HashMap::new();
    // The parser joined the left side so the parse result has a nested tuple adding left like
    // (((#first, #second), #third), #forth)..., and getting it by pattern matching, generate the tuple.
    let tuple_pattern = generate_tuple_pattern(&mut counter, &field_names);
    let struct_constructor = quote! {
        #struct_name {
            #(#field_names),*
        }
    };

    let final_parser = quote! {
        #generated_full_parser.map(|#tuple_pattern| #struct_constructor)
    };

    final_parser
}

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
            chumsky::prelude::just(#next_literal)
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
                .map_err(|e| chumsky::error::Rich::custom(
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
