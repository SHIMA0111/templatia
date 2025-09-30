use crate::parser::TemplateSegments;
use crate::utils::{get_field_type_by_name, get_type_name, is_allowed_consecutive_allowed_type, CONSECUTIVE_PLACEHOLDER_ALLOWED_TYPE};

pub(crate) fn validate_template_safety(
    segments: &[TemplateSegments],
    all_fields: &[syn::Field],
) -> Result<(), String> {
    for window in segments.windows(2) {
        if let [TemplateSegments::Placeholder(first), TemplateSegments::Placeholder(second)] = window {
            let first_type = get_field_type_by_name(first, all_fields);

            let (allowed_consecutive, first_type_name) = match first_type {
                Some(first_ty) => {
                    (is_allowed_consecutive_allowed_type(first_ty), get_type_name(first_ty))
                },
                _ => (false, "unknown_type".to_string()),
            };

            if !allowed_consecutive {
                return Err(
                    format!(
                        "Placeholder \"{0}\" and \"{1}\" is consecutive. These are ambiguous to parsing.\
                        \n\"{0}\" is `{2}` type data. Consecutive allows only: [{3}]",
                        first, second, first_type_name, CONSECUTIVE_PLACEHOLDER_ALLOWED_TYPE.join(", ")
                    )
                )
            }
        }
    }

    Ok(())
}