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
    // The type needs to be a Type::Path variant
    if let syn::Type::Path(type_path) = ty {
        // Getting the last segment of the path
        // This is robust for such as std::option::Option which has multiple segments
        if let Some(last_segment) = type_path.path.segments.last() {
            // Checking if the last segment is Option
            if last_segment.ident == "Option" {
                // Checking if the Option has a generic argument.
                // The AngleBracketed indicates `<...>` part of Option<T>
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    // Validating that the Option has only one generic argument
                    if args.args.len() == 1 {
                        // Validating that the generic argument is a type, and extracting it
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                            return Some(inner_type);
                        }
                    }
                }
            }
        }
    }
    None
}