use std::error::Error;

pub trait Template {
    type Error: Error;
    type Struct;

    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Result<Self::Struct, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Inconsistent values for placeholder '{placeholder}': found '{first_value}', and after face '{second_value}'")]
    InconsistentValues {
        placeholder: String,
        first_value: String,
        second_value: String,
    },
    #[error("Parse error: {0}")]
    Parse(String),
}