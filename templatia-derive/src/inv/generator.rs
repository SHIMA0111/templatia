use crate::error::generate_not_found_placeholder_compile_error;
use crate::fields::{FieldKind, Fields};
use crate::inv::parser::generate_parser_from_segments;
use crate::inv::validator::validate_template_safety;
use crate::parser::TemplateSegments;
use quote::quote;
use std::collections::{HashMap, HashSet};

pub(crate) fn generate_str_parser(
    struct_name: &syn::Ident,
    fields: &Fields,
    placeholder_names: &HashSet<String>,
    segments: &[TemplateSegments],
    allow_missing_placeholders: bool,
    empty_str_as_none: bool,
    escaped_colon_marker: &str,
) -> proc_macro2::TokenStream {
    for name in placeholder_names {
        if !fields.field_names().contains(name) {
            return generate_not_found_placeholder_compile_error(
                struct_name.to_string().as_str(),
                name,
            );
        }
    }

    if let Err(e) = validate_template_safety(segments, fields) {
        return e;
    }

    let replace_colon = quote! { replace(":", #escaped_colon_marker) };
    let generated_full_parser =
        generate_parser_from_segments(segments, fields, empty_str_as_none, &replace_colon);

    let field_names = segments
        .iter()
        .filter_map(|segment| match segment {
            TemplateSegments::Placeholder(name) => {
                Some(syn::Ident::new(name, proc_macro2::Span::call_site()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    // The parser joined the left side so the parse result has a nested tuple adding left like
    // (((#first, #second), #third), #forth)..., and getting it by pattern matching, generate the tuple.
    // And also, the template can have a duplicate key so the vec for the duplication checks is also returned.
    let (tuple_pattern, dup_checks) = generate_tuple_pattern(&field_names);

    // Unique field names included in the template
    let unique_field_names_in_placeholder = placeholder_names
        .iter()
        .map(|name| syn::Ident::new(name, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();

    let (missing_placeholders_option, missing_placeholders_non_option) =
        fields.missing_placeholders_sep_opt(placeholder_names);

    // Even if the template has no all fields without allow_missing_placeholders,
    // it is passed if the missing_placeholders are Option<T> type
    if !allow_missing_placeholders && !missing_placeholders_non_option.is_empty() {
        let error = syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "{} has more field specified than the template's placeholders: {}\n\
                If you want to allow missing placeholders, \
                use `#[templatia(allow_missing_placeholders)]` attribute.",
                struct_name,
                missing_placeholders_non_option
                    .iter()
                    .map(|ident| ident.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
        return error.to_compile_error();
    }

    let struct_constructor = quote! {
        #struct_name {
            // #(#Awesome,)* will be expanded to #Awesome, #Awesome, #Awesome <- This is the correct behavior.
            // #(#Awesome),* will be expanded to #Awesome, #Awesome
            //  - BAD implementation. unique_field_names is not empty, and the missing_placeholders is also empty,
            //    the comma of the last element from the unique_field_names not be added comma,
            //    so the next element from the missing_placeholders returns error.
            // #(#Awesome),*, will be expanded to #Awesome, #Awesome,... but even if the element is empty, the comma is still there. This causes the error.
            #(#unique_field_names_in_placeholder,)*
            #(#missing_placeholders_non_option: Default::default(),)*
            #(#missing_placeholders_option: None,)*
        }
    };

    // Generate duplicate check code that expands to base_value != dup_value.
    // At execution time, the comparison operation is statically determined. In most cases,
    // static comparison is more efficient than dynamic comparison.
    // To ensure duplicate placeholders don't receive different values,
    // all duplicate placeholders must be checked.
    // If there are N duplicate placeholders, this comparison approach is O(N).
    // Using dynamic comparison does not appear to reduce this complexity.
    let dup_conditions = dup_checks
        .iter()
        .map(|(base, dup, _)| quote! { #dup != #base });
    let dup_names = dup_checks.iter().map(|(_, _, name)| {
        quote! { #name }
    });

    let dup_bases = dup_checks.iter().map(|(base, _, name)| {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        match fields.get_field_kind(&ident) {
            Some(FieldKind::Option(_)) => quote! {
                #base
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            },
            Some(FieldKind::Vec(_)) | Some(FieldKind::BTreeSet(_)) => quote! {
                #base
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            },
            Some(FieldKind::HashSet(_)) => quote! {
                #base
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(",")
            },
            _ => quote! { #base },
        }
    });
    let dup_dups = dup_checks.iter().map(|(_, dup, name)| {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());

        match fields.get_field_kind(&ident) {
            Some(FieldKind::Option(_)) => quote! {
                #dup
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            },
            Some(FieldKind::Vec(_)) | Some(FieldKind::BTreeSet(_)) => quote! {
                #dup
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            },
            Some(FieldKind::HashSet(_)) => quote! {
                #dup
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(",")
            },
            _ => quote! { #dup },
        }
    });

    let final_parser = quote! {
        #generated_full_parser
            .try_map(|#tuple_pattern, span| {
            #(
                if #dup_conditions {
                    return Err(::templatia::__private::chumsky::error::Rich::custom(
                        span,
                        format!(
                            "__templatia_conflict__:{}::{}::{}",
                            #dup_names.#replace_colon,
                            #dup_bases.to_string().#replace_colon,
                            #dup_dups.to_string().#replace_colon,
                        )
                    ));
                }
            )*
            Ok(#struct_constructor)
        })
    };

    final_parser
}

fn generate_tuple_pattern(
    field_names: &[syn::Ident],
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
