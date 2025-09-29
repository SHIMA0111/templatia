//! # Templatia Derive
//!
//! Procedural macros for the templatia template parsing library.
//!
//! This crate provides the `#[derive(Template)]` macro that automatically generates
//! `templatia::Template` trait implementations for named structs.
//!
//! ## Limitations
//!
//! - **Named Structs Only**: Currently only `struct Name { field: Type }` is supported
//! - **No Tuple Structs**: `struct Point(i32, i32)` is not supported yet
//! - **No Enums**: Enum support is planned for future versions
//! - **Field Requirements**: Template fields must implement `Display`, `FromStr`, and `PartialEq`
//!
//! ## Attribute Reference
//!
//! ### `#[templatia(template = "...")]`
//!
//! Defines a custom template string with `{field_name}` placeholders.
//!
//! **Rules:**
//! - Placeholders must match struct field names exactly
//! - All placeholders must reference existing fields
//! - Duplicate placeholders are allowed but must have consistent values during parsing
//!
//! For detailed usage examples and comprehensive documentation, see the main `templatia` crate.

mod generator;
mod parser;

use crate::generator::generate_str_parser;
use crate::parser::{TemplateSegments, parse_template};
use darling::FromDeriveInput;
use darling::util::Override;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{DeriveInput, parse_macro_input};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(templatia), supports(struct_named))]
struct TemplateOpts {
    /// The target struct identifier.
    ident: syn::Ident,
    /// All fields of the target struct.
    data: darling::ast::Data<(), syn::Field>,
    /// Optional template string provided via `#[templatia(template = "...")]`.
    #[darling(default)]
    template: Override<String>,
}

/// Derive macro for implementing `templatia::Template` trait on named structs.
///
/// This procedural macro automatically generates `Template` trait implementations,
/// enabling bidirectional conversion between structs and template strings.
///
/// # Type Requirements
///
/// All fields referenced in the template must implement:
/// - `std::fmt::Display` for serialization (`to_string`)
/// - `std::str::FromStr` for deserialization (`from_string`)
/// - `std::cmp::PartialEq` for consistency validation with duplicate placeholders
///
/// # Compilation Errors
///
/// The macro will produce compile-time errors in the following cases:
/// - Template references non-existent struct fields
/// - Template parsing fails due to invalid syntax
/// - Applied to unsupported struct types (tuple structs, unit structs, enums)
/// - Field types don't satisfy the required trait bounds
#[proc_macro_derive(Template, attributes(templatia))]
pub fn template_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let opts = match TemplateOpts::from_derive_input(&ast) {
        Ok(opts) => opts,
        Err(e) => return e.write_errors().into(),
    };

    let name = &opts.ident;

    let template = match &opts.template {
        Override::Explicit(template) => template.to_string(),
        Override::Inherit => {
            if let syn::Data::Struct(data_struct) = &ast.data {
                if let syn::Fields::Named(fields_named) = &data_struct.fields {
                    fields_named
                        .named
                        .iter()
                        .filter_map(|field| field.ident.as_ref())
                        .map(|ident| format!("{0} = {{{0}}}", ident.to_string()))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }
    };

    let segments = match parse_template(&template) {
        Ok(segments) => segments,
        Err(e) => {
            let error =
                syn::Error::new_spanned(&opts.ident, format!("Failed to parse template: {}", e));
            // Transform syn::Error to TokenStream, and fast return
            return error.to_compile_error().into();
        }
    };

    // Generate format string like "key = {}, key2 = {}"
    let format_string = segments
        .iter()
        .map(|segment| match segment {
            TemplateSegments::Literal(lit) => lit.replace("{", "{{").replace("}", "}}"),
            TemplateSegments::Placeholder(_) => "{}".to_string(),
        })
        .collect::<String>();

    // Generate code for placeholder completion the format_string it used the self keys
    let format_args = segments.iter().filter_map(|segment| match segment {
        TemplateSegments::Placeholder(name) => {
            let field_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            Some(quote! { &self.#field_ident })
        }
        TemplateSegments::Literal(_) => None,
    });

    let all_fields = if let darling::ast::Data::Struct(data_struct) = &opts.data {
        &data_struct.fields
    } else {
        // Currently, this crates supports only named struct so this branch is unreachable.
        unreachable!()
    };

    // Gathering the all placeholder name without duplication
    let placeholder_names = segments
        .iter()
        .filter_map(|segment| {
            if let TemplateSegments::Placeholder(name) = segment {
                Some(name.trim().to_string())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    let str_from_parser = generate_str_parser(name, all_fields, &placeholder_names, &segments);

    // Generate trait bound
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let used_fields = all_fields
        .iter()
        .filter(|field| {
            if let Some(ident) = &field.ident {
                placeholder_names.contains(&ident.to_string())
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    let mut new_where_clause = where_clause
        .cloned()
        .unwrap_or_else(|| syn::parse_quote! { where });
    for field in used_fields {
        let field_ty = &field.ty;
        if !new_where_clause.predicates.is_empty() {
            new_where_clause.predicates.push_punct(Default::default());
        }
        new_where_clause.predicates.push(syn::parse_quote! {
            #field_ty: ::std::fmt::Display + ::std::str::FromStr + ::std::cmp::PartialEq
        });
        new_where_clause.predicates.push(syn::parse_quote! {
            <#field_ty as ::std::str::FromStr>::Err: ::std::fmt::Display
        })
    }

    let where_clause = if new_where_clause.predicates.is_empty() {
        quote! {}
    } else {
        quote! { #new_where_clause }
    };

    quote! {
        impl #impl_generics ::templatia::Template for #name #ty_generics #where_clause {
            type Error = templatia::TemplateError;
            type Struct = #name #ty_generics;

            fn to_string(&self) -> String {
                format!(#format_string, #(#format_args),*)
            }

            fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
                use ::templatia::__private::chumsky::Parser;

                let parser = #str_from_parser;
                match parser.parse(s).into_result() {
                    Ok(value) => Ok(value),
                    Err(errs) => {
                        for err in &errs {
                            if let ::templatia::__private::chumsky::error::RichReason::Custom(msg) = err.reason() {
                            let m = msg.to_string();
                            const PFX: &str = "__templatia_conflict__:";
                            if let Some(rest) = m.strip_prefix(PFX) {
                                if let Some((placeholder, rest)) = rest.split_once("::") {
                                    if let Some((first_value, second_value)) = rest.split_once("::") {
                                        return Err(templatia::TemplateError::InconsistentValues {
                                            placeholder: placeholder.to_string(),
                                            first_value: first_value.to_string(),
                                            second_value: second_value.to_string(),
                                        });
                                    }
                                }
                            }}
                        }

                        let error_message = errs.into_iter()
                            .map(|err| err.to_string())
                            .collect::<Vec<_>>()
                            .join("\n");

                        Err(templatia::TemplateError::Parse(error_message))
                    }
                }
            }
        }
    }.into()
}
