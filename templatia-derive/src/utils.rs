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