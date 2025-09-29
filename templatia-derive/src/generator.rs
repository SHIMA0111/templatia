use crate::parser::TemplateSegments;
use quote::quote;
use std::collections::{HashMap, HashSet};

pub(crate) fn generate_str_parser(
    struct_name: &syn::Ident,
    all_fields: &[syn::Field],
    placeholder_names: &HashSet<String>,
    segments: &[TemplateSegments],
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
                format!(
                    "{} has no field named \"{}\"",
                    struct_name.to_string(),
                    name
                ),
            );

            return error.to_compile_error().into();
        }
    }

    let mut parsers = segments.iter().peekable();
    let mut generated_full_parser = quote! { ::templatia::__private::chumsky::prelude::empty() };
    let mut parser_count = 0;
    let mut latest_segment_was_literal = false;

    while let Some(segment) = parsers.next() {
        match segment {
            TemplateSegments::Literal(lit) => {
                if parser_count == 0 {
                    generated_full_parser = quote! {
                    ::templatia::__private::chumsky::prelude::just::<&str, &str, ::templatia::__private::chumsky::extra::Err<::templatia::__private::chumsky::error::Rich<char>>>(#lit).ignored() }
                } else {
                    generated_full_parser = quote! { #generated_full_parser.then_ignore(::templatia::__private::chumsky::prelude::just(#lit)) }
                }
                latest_segment_was_literal = true;
            }
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
                            format!(
                                "{} has no field named \"{}\"",
                                struct_name.to_string(),
                                name
                            ),
                        );
                        return err.to_compile_error().into();
                    }
                };

                let field_type = &field.ty;
                let field_parser =
                    generate_field_parser(&name_ident, field_type, parsers.peek().cloned());

                if parser_count == 0 {
                    generated_full_parser = field_parser;
                } else {
                    if latest_segment_was_literal && parser_count == 1 {
                        generated_full_parser =
                            quote! { #generated_full_parser.ignore_then(#field_parser) }
                    } else {
                        generated_full_parser =
                            quote! { #generated_full_parser.then(#field_parser) }
                    }
                }
                latest_segment_was_literal = false;
            }
        }
        // Count of Literal parser count
        parser_count += 1;
    }

    generated_full_parser = quote! { #generated_full_parser.then_ignore(::templatia::__private::chumsky::prelude::end()) };

    let field_names = segments
        .iter()
        .filter_map(|segment| match segment {
            TemplateSegments::Placeholder(name) => {
                Some(syn::Ident::new(&name, proc_macro2::Span::call_site()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    // The parser joined the left side so the parse result has a nested tuple adding left like
    // (((#first, #second), #third), #forth)..., and getting it by pattern matching, generate the tuple.
    // And also, the template can have a duplicate key so the vec for the duplication checks is also returned.
    let (tuple_pattern, dup_checks) = generate_tuple_pattern(&field_names);

    // Construct the struct from unique field names (placeholders may repeat in the template)
    let mut seen_fields = HashSet::new();
    let unique_field_names = field_names
        .iter()
        .filter(|ident| seen_fields.insert(ident.to_string()))
        .collect::<Vec<_>>();

    let struct_constructor = quote! {
        #struct_name {
            #(#unique_field_names),*
        }
    };

    let dup_conds = dup_checks
        .iter()
        .map(|(base, dup, _)| quote! { #dup != #base });
    let dup_names = dup_checks.iter().map(|(_, _, name)| quote! { #name });
    let dup_bases = dup_checks.iter().map(|(base, _, _)| quote! { #base });
    let dup_dups = dup_checks.iter().map(|(_, dup, _)| quote! { #dup });

    let final_parser = quote! {
        #generated_full_parser
            .try_map(|#tuple_pattern, span| {
            #(
                if #dup_conds {
                    return Err(::templatia::__private::chumsky::error::Rich::custom(
                        span,
                        format!(
                            "__templatia_conflict__:{}::{}::{}",
                            #dup_names, #dup_bases, #dup_dups
                        )
                    ));
                }
            )*
            Ok(#struct_constructor)
        })
    };

    final_parser
}

fn generate_field_parser(
    field_name: &syn::Ident,
    field_type: &syn::Type,
    next_segment: Option<&TemplateSegments>,
) -> proc_macro2::TokenStream {
    let next_literal = match next_segment {
        Some(TemplateSegments::Literal(lit)) => Some(lit),
        _ => None,
    };

    let value_extractor = if let Some(next_literal) = next_literal {
        quote! {
            ::templatia::__private::chumsky::prelude::just::<&str, &str, ::templatia::__private::chumsky::extra::Err<::templatia::__private::chumsky::error::Rich<char>>>(#next_literal)
                .not()
                .ignore_then(::templatia::__private::chumsky::prelude::any())
                .repeated()
                .to_slice()
        }
    } else {
        quote! {
            ::templatia::__private::chumsky::prelude::any::<&str, ::templatia::__private::chumsky::extra::Err<::templatia::__private::chumsky::error::Rich<char>>>().repeated().to_slice()
        }
    };

    // CAUTION: In this generator, the try_map isn't called to the TokenStream; it calls the chumsky Object generated from to_slice().
    quote! {
        #value_extractor.try_map(|s: &str, span| {
            s.parse::<#field_type>()
                .map_err(|e| ::templatia::__private::chumsky::error::Rich::<char>::custom(
                    span,
                    format!("Failed to parse field \"{}\": {}", stringify!(#field_name), e)
                ))
        })
    }
}

fn generate_tuple_pattern(
    field_names: &Vec<syn::Ident>,
) -> (
    proc_macro2::TokenStream,
    Vec<(syn::Ident, syn::Ident, String)>,
) {
    let mut first_binds: HashMap<String, syn::Ident> = HashMap::new();
    let mut dup_checks: Vec<(syn::Ident, syn::Ident, String)> = Vec::new();

    let mut seen_field_names: HashMap<String, usize> = HashMap::new();
    let mut key_generator = |key: &syn::Ident| -> syn::Ident {
        let res = seen_field_names
            .entry(key.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);

        if *res > 1 {
            let new_key = format!("_{}_{}", key, res);
            let dup_ident = syn::Ident::new(&new_key, proc_macro2::Span::call_site());
            let base_ident = first_binds
                .get(&key.to_string())
                .cloned()
                .unwrap_or_else(|| key.clone());

            dup_checks.push((base_ident, dup_ident.clone(), key.to_string()));
            dup_ident
        } else {
            first_binds.insert(key.to_string(), key.clone());
            key.clone()
        }
    };

    let tuple_pat = if !field_names.is_empty() {
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
    };

    (tuple_pat, dup_checks)
}
