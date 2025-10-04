use syn::GenericArgument;
use syn::punctuated::Punctuated;

pub(crate) const CONSECUTIVE_PLACEHOLDER_ALLOWED_TYPE: [&str; 2] = ["char", "bool"];

pub(crate) fn is_allowed_consecutive_allowed_type(field_type: &syn::Type) -> bool {
    match field_type {
        syn::Type::Path(path) => {
            if let Some(ident) = &path.path.get_ident() {
                CONSECUTIVE_PLACEHOLDER_ALLOWED_TYPE.contains(&ident.to_string().as_str())
            } else {
                false
            }
        },
        _ => false,
    }
}

pub(crate) fn get_field_type_by_name<'a>(field_name: &str, all_fields: &'a [syn::Field]) -> Option<&'a syn::Type> {
    all_fields
        .iter()
        .find(|field| {
            field.ident.as_ref().map(|ident| ident.to_string()) == Some(field_name.to_string())
        })
        .map(|field| &field.ty)
}

pub(crate) fn get_type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => {
            if let Some(ident) = &path.path.get_ident() {
                ident.to_string()
            } else {
                "unknown_type".to_string()
            }
        },
        _ => "unknown_type".to_string(),
    }
}

// Extracts the inner type from Option<T> to Option<&syn::Type = T>
pub(crate) fn get_inner_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    if let Some((ident, punctuated_comma)) = get_inner_type_from_generic(ty) {
        if ident == "Option" {
            // Validating that the Option has only one generic argument
            if punctuated_comma.len() == 1 {
                if let Some(GenericArgument::Type(ty)) = punctuated_comma.first() {
                    return Some(ty);
                }
            }
        }
    }
    None
}

/// Gets the inner type from a generic type like Option<T> or Vec<T>
/// Returns the last segment of the path and the generic arguments
fn get_inner_type_from_generic(ty: &syn::Type) -> Option<(&syn::Ident, &Punctuated<GenericArgument, syn::Token![,]>)> {
    // The type needs to be a Type::Path variant. Path variants are used for named paths.
    if let syn::Type::Path(type_path) = ty {
        // Getting the last segment of the path,
        // This is robust for such as std::option::Option which has multiple segments
        if let Some(last_segment) = type_path.path.segments.last() {
            // Getting the generic arguments of the last segment.
            if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                return Some((&last_segment.ident, &args.args));
            }
        }
    }
    None
}