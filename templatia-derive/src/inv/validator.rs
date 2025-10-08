use crate::error::generate_consecutive_compile_error;
use crate::fields::{FieldKind, Fields};
use crate::parser::TemplateSegments;
use crate::utils::{get_type_name, is_allowed_consecutive_allowed_type};

pub(crate) fn validate_template_safety(
    segments: &[TemplateSegments],
    fields: &Fields,
) -> Result<(), proc_macro2::TokenStream> {
    for window in segments.windows(2) {
        if let [TemplateSegments::Placeholder(first), TemplateSegments::Placeholder(second)] = window {
            let first_type = fields.get_type_kind_by_name(first);
            let (allowed_consecutive, first_type_name) = match first_type {
                Some(FieldKind::Option(ty)) => {
                    (is_allowed_consecutive_allowed_type(ty), format!("Option<{}>", get_type_name(ty)))
                },
                Some(FieldKind::Primitive(ty)) => {
                    (is_allowed_consecutive_allowed_type(ty), get_type_name(ty))
                },
                _ => (false, "unknown_type".to_string()),
            };

            if !allowed_consecutive {
                return Err(generate_consecutive_compile_error(first, second, first_type_name.as_str()))
            }
        }
    }

    Ok(())
}