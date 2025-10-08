use std::collections::HashSet;
use proc_macro2::TokenStream;
use quote::quote;
use crate::parser::TemplateSegments;

pub(super) fn generate_format_string_args(
    segments: &[TemplateSegments<'_>],
    option_fields: &HashSet<&syn::Ident>,
) -> (String, Vec<TokenStream>)
{
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
                if option_fields.contains(&&field_ident) {
                    Some( quote! { &self.#field_ident.as_ref().map(|v| v.to_string()).unwrap_or("".to_string()) } )
                } else {
                    Some( quote! { &self.#field_ident } )
                }
            },
            TemplateSegments::Literal(_) => None,
        }).collect::<Vec<_>>();

    (format_string, format_args)
}