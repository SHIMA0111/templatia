use crate::parser::TemplateSegments;
use proc_macro2::TokenStream;
use quote::quote;
use crate::error::{generate_not_found_placeholder_compile_error, generate_unsupported_compile_error};
use crate::fields::{FieldKind, Fields};

pub(super) fn generate_format_string_args(
    segments: &[TemplateSegments<'_>],
    fields: &Fields,
) -> (String, Vec<TokenStream>) {
    // Generate format string like "key = {}, key2 = {}"
    let format_string = segments
        .iter()
        .map(|segment| match segment {
            TemplateSegments::Literal(lit) => lit.replace("{", "{{").replace("}", "}}"),
            TemplateSegments::Placeholder(_) => "{}".to_string(),
        })
        // This collect works because the String implements FromIterator.
        .collect::<String>();

    // Generate code for placeholder completion the format_string it used the self keys
    let format_args = segments
        .iter()
        .filter_map(|segment| match segment {
            TemplateSegments::Placeholder(name) => {
                let field_ident = syn::Ident::new(name, proc_macro2::Span::call_site());

                // &self.#field_ident means the field of the struct named `field_ident`
                // If the struct is
                // ```rust
                // struct Point { x: i32, y: i32 }
                // ```
                // then the field_ident is `x` or `y`.
                // The token stream indicates &self.x or &self.y.
                // Please note: the #field_ident is not `field_ident` but `x` or `y`.
                match fields.get_field_kind(&field_ident) {
                    Some(ty) => match ty {
                        FieldKind::Option(_) => {
                            Some(quote! {
                                &self.#field_ident.as_ref().map(|v| v.to_string()).unwrap_or_else(|| String::new())
                            })
                        },
                        FieldKind::Vec(_) => {
                            Some(quote! {
                                &self.#field_ident.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
                            })
                        },
                        FieldKind::HashSet(_) => {
                            Some(quote! {
                                &self.#field_ident.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
                            })
                        },
                        FieldKind::BTreeSet(_) => {
                            Some(quote! {
                                &self.#field_ident.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
                            })
                        },
                        FieldKind::Primitive(_) => {
                            Some(quote! {
                                &self.#field_ident
                            })
                        },
                        _ => {
                            Some(generate_unsupported_compile_error(&field_ident, ty))
                        },
                    },
                    _ => Some(generate_not_found_placeholder_compile_error("struct", name))
                }
            },
            TemplateSegments::Literal(_) => None,
        }).collect::<Vec<_>>();

    (format_string, format_args)
}
