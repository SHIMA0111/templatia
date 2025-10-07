use crate::fields::FieldKind;
use crate::utils::get_type_name;

pub(crate) fn generate_compile_error(msg: &str) -> proc_macro2::TokenStream {
    let error = syn::Error::new(proc_macro2::Span::call_site(), msg);
    error.to_compile_error().into()
}

pub(crate) fn generate_unsupported_compile_error(field: &syn::Ident, ty: &FieldKind) -> proc_macro2::TokenStream {
    let msg = format!(
        "unsupported type field: {0} has a {1} type. currently, {1} is not supported.",
        // Currently, support only named struct so this unwrap is safe.
        field.to_string(),
        match ty {
            FieldKind::Result(ok_ty, err_ty) => format!(
                "Result<{}, {}>", get_type_name(ok_ty), get_type_name(err_ty)
            ),
            FieldKind::Vec(ty) => format!(
                "Vec<{}>", get_type_name(ty),
            ),
            FieldKind::HashSet(ty) => format!(
                "HashSet<{}>", get_type_name(ty),
            ),
            FieldKind::BTreeSet(ty) => format!(
                "BTreeSet<{}>", get_type_name(ty),
            ),
            FieldKind::HashMap(k_ty, v_ty) => format!(
                "HashMap<{}, {}>", get_type_name(k_ty), get_type_name(v_ty),
            ),
            FieldKind::BTreeMap(k_ty, v_ty) => format!(
                "BTreeMap<{}, {}>", get_type_name(k_ty), get_type_name(v_ty),
            ),
            FieldKind::Tuple => "tuple".to_string(),
            FieldKind::Unknown => "cannot recognize the field".to_string(),
            _ => unreachable!(),
        }
    );

    generate_compile_error(&msg)
}