use crate::error::generate_unsupported_compile_error;
use crate::fields::{FieldKind, Fields};
use crate::parser::TemplateSegments;
use crate::utils::get_type_name;
use quote::quote;
use std::collections::HashMap;

pub(crate) fn generate_parser_from_segments(
    segments: &[TemplateSegments],
    fields: &Fields,
    empty_str_as_none: bool,
    colon_escaper: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut peekable_segments = segments.iter().peekable();
    let mut parser = quote! { ::templatia::__private::chumsky::prelude::empty() };

    let mut is_first_segment = true;
    let mut is_passed_first_placeholder = false;
    let mut latest_segment_was_literal = false;

    let mut literals_counters = HashMap::new();
    let mut last_literal_parsed: &str = "";
    let mut last_literal_count: i32 = -1;

    while let Some(segment) = peekable_segments.next() {
        match segment {
            TemplateSegments::Literal(lit) => {
                let count = *literals_counters
                    .entry(lit)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                parser = if is_first_segment {
                    quote! {
                        just::<&str, &str, chumsky::extra::Err<chumsky::error::Rich<char>>>(#lit)
                            .ignored()
                    }
                } else {
                    quote! { #parser.then_ignore(just(#lit)) }
                };

                parser = quote! {
                    #parser
                        .map_err(|e| {
                            let start = match e.found() {
                                Some(_) => {
                                    e.span().start
                                },
                                None => {
                                    if #last_literal_count > 0 {
                                        let found_lit = s.match_indices(#last_literal_parsed).collect::<Vec<_>>();
                                        // SAFETY: In this branch, the last_literal_count is always 1 or more. So, the #last_literal_count - 1 is always converted to usize.
                                        // Also, the last_literal_parsed and last_literal_count indicate **last**, so in this branch executed,
                                        // the last literal is parsed, so the index(last_literal_count - 1) is always less than the length of the s.match_indices(#last_literal_parsed).collect::<Vec<_>>().
                                        // Therefore, the following code never causes an out-of-range panic.
                                        let (last_indices, _) = found_lit[(#last_literal_count - 1) as usize];
                                        last_indices + #last_literal_parsed.len()
                                    } else {
                                        0usize
                                    }
                                }
                            };

                            chumsky::error::Rich::<char>::custom(
                                e.span().clone(),
                                // SAFETY: The start is 0 or index from the s. Therefore, this isn't an out of range.
                                format!("__templatia_parse_literal__:{}::{}",
                                    #lit.#colon_escaper,
                                    &s[start..].#colon_escaper,
                                )
                            )
                        })
                };

                latest_segment_was_literal = true;
                last_literal_parsed = lit;
                last_literal_count = count;
            }
            TemplateSegments::Placeholder(placeholder) => {
                let name_ident = syn::Ident::new(placeholder, proc_macro2::Span::call_site());

                // SAFETY: The placeholder is always in the fields because in the first of the generate_str_parser,
                // the placeholder is checked if it is in the fields.
                let field_kind = fields.get_field_kind(&name_ident).unwrap();

                let field_parser = generate_field_parser(
                    &name_ident,
                    field_kind,
                    peekable_segments.peek().cloned(),
                    empty_str_as_none,
                    colon_escaper,
                );

                if is_first_segment {
                    parser = field_parser;
                } else if !is_passed_first_placeholder && latest_segment_was_literal {
                    parser = quote! { #parser.ignore_then(#field_parser) };
                } else {
                    parser = quote! { #parser.then(#field_parser) };
                }

                is_passed_first_placeholder = true;
                latest_segment_was_literal = false;
            },
            // TODO: support group box
            TemplateSegments::GroupBox { segments: _, placeholder: _ } => {}
        }
        is_first_segment = false;
    }

    quote! { #parser.then_ignore(end()) }
}

fn generate_field_parser(
    field_name: &syn::Ident,
    field_type: &FieldKind,
    next_segment: Option<&TemplateSegments>,
    empty_str_as_none: bool,
    colon_escaper: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let next_literal = match next_segment {
        Some(TemplateSegments::Literal(lit)) => Some(*lit),
        _ => None,
    };

    let field_type_str = field_type.to_string();
    match field_type {
        FieldKind::Option(ty) => {
            let is_string_type =
                matches!(get_type_name(ty).to_lowercase().as_str(), "string" | "str");
            let inner_parser = generate_parser(ty, next_literal);

            quote! {
                #inner_parser
                    .try_map(|s: &str, span| {
                        if (#empty_str_as_none || !#is_string_type) && s.is_empty() {
                            Ok(None)
                        } else {
                            s.parse::<#ty>()
                                .map(Some)
                                .map_err(|_| {
                                    chumsky::error::Rich::<char>::custom(
                                        span,
                                        format!(
                                            "__templatia_parse_type__:{}::{}::{}",
                                            stringify!(#field_name).#colon_escaper,
                                            s.#colon_escaper,
                                            #field_type_str.#colon_escaper,
                                        )
                                    )
                                })
                        }
                    })
            }
        }
        FieldKind::Vec(ty) => {
            let inner_parser = generate_str_parser(next_literal);

            quote! {
                #inner_parser
                    .try_map(|s: &str, span| {
                        let mut vec = Vec::new();
                        if s.is_empty() {
                            Ok(vec)
                        } else {
                            let values = s.split(',');

                            for value in values {
                                match value.parse::<#ty>() {
                                    Ok(v) => {
                                        vec.push(v);
                                    },
                                    Err(_) => {
                                        // I'm not sure if this way is the best for the collection parser.
                                        // However, this way works for now.
                                        return Err(chumsky::error::Rich::<char>::custom(
                                            span,
                                            format!(
                                                "__templatia_parse_type__:{}::{}::{}",
                                                stringify!(#field_name).#colon_escaper,
                                                s.#colon_escaper,
                                                #field_type_str.#colon_escaper,
                                            )
                                        ))
                                    }
                                }
                            }
                            Ok(vec)
                        }
                    })
            }
        }
        FieldKind::HashSet(ty) => {
            let inner_parser = generate_str_parser(next_literal);

            quote! {
                #inner_parser
                    .try_map(|s: &str, span| {
                        let mut set = std::collections::HashSet::new();
                        if s.is_empty() {
                            Ok(set)
                        } else {
                            let values = s.split(',');

                            for value in values {
                                match value.parse::<#ty>() {
                                    Ok(v) => {
                                        set.insert(v);
                                    },
                                    Err(_) => {
                                        return Err(chumsky::error::Rich::<char>::custom(
                                            span,
                                            format!(
                                                "__templatia_parse_type__:{}::{}::{}",
                                                stringify!(#field_name).#colon_escaper,
                                                s.#colon_escaper,
                                                #field_type_str.#colon_escaper,
                                            )
                                        ))
                                    }
                                }
                            }
                            Ok(set)
                        }
                    })
            }
        }
        FieldKind::BTreeSet(ty) => {
            let inner_parser = generate_str_parser(next_literal);

            quote! {
                #inner_parser
                    .try_map(|s: &str, span| {
                        let mut b_set = std::collections::BTreeSet::new();
                        if s.is_empty() {
                            Ok(b_set)
                        } else {
                            let values = s.split(',');

                            for value in values {
                                match value.parse::<#ty>() {
                                    Ok(v) => {
                                        b_set.insert(v);
                                    },
                                    Err(_) => {
                                        return Err(chumsky::error::Rich::<char>::custom(
                                            span,
                                            format!(
                                                "__templatia_parse_type__:{}::{}::{}",
                                                stringify!(#field_name).#colon_escaper,
                                                s.#colon_escaper,
                                                #field_type_str.#colon_escaper,
                                            )
                                        ))
                                    }
                                }
                            }
                            Ok(b_set)
                        }
                    })
            }
        }
        FieldKind::Primitive(ty) => {
            let parser = generate_parser(ty, next_literal);

            quote! {
                #parser
                    .try_map(|s: &str, span| {
                        s.parse::<#ty>()
                            .map_err(|_| {
                                chumsky::error::Rich::<char>::custom(
                                    span,
                                    format!(
                                        "__templatia_parse_type__:{}::{}::{}",
                                        stringify!(#field_name).#colon_escaper,
                                        s.#colon_escaper,
                                        #field_type_str.#colon_escaper,
                                    )
                                )
                            })
                    })
            }
        }
        _ => generate_unsupported_compile_error(field_name, field_type),
    }
}

fn generate_parser(field_type: &syn::Type, next_literal: Option<&str>) -> proc_macro2::TokenStream {
    let base_parser = generate_base_parser(next_literal);

    match get_type_name(field_type).as_str() {
        "char" => quote! {
            any::<&str, chumsky::extra::Err<chumsky::error::Rich<char>>>()
                .map(|c| c.to_string())
                .to_slice()
        },
        "bool" => quote! {
            choice((
                just::<&str, &str, chumsky::extra::Err<chumsky::error::Rich<char>>>("true").to_slice(),
                just::<&str, &str, chumsky::extra::Err<chumsky::error::Rich<char>>>("false").to_slice(),
                #base_parser.at_most(5).to_slice(),
            ))
        },
        _ => quote! {
            #base_parser.to_slice()
        },
    }
}

fn generate_str_parser(next_literal: Option<&str>) -> proc_macro2::TokenStream {
    let base_parser = generate_base_parser(next_literal);
    quote! {
        #base_parser.to_slice()
    }
}

fn generate_base_parser(next_literal: Option<&str>) -> proc_macro2::TokenStream {
    if let Some(next_lit) = next_literal {
        quote! {
            just::<&str, &str, chumsky::extra::Err<chumsky::error::Rich<char>>>(#next_lit)
                .not()
                .ignore_then(any())
                .repeated()
        }
    } else {
        quote! {
            any::<&str, chumsky::extra::Err<chumsky::error::Rich<char>>>()
                .repeated()
        }
    }
}
