use templatia::Template;
use templatia::TemplateError;

// Tests in this file follow AGENTS.md: they reflect the documented intent.
// Do not modify tests to match implementation if they fail.

#[test]
fn template_error_parse_displays_message() {
    let e = TemplateError::Parse("bad input".to_string());
    assert_eq!(e.to_string(), "Parse error: bad input");
}

#[test]
fn template_error_inconsistent_values_includes_fields() {
    let e = TemplateError::InconsistentValues {
        placeholder: "name".to_string(),
        first_value: "a".to_string(),
        second_value: "b".to_string(),
    };
    // Ensure the formatted string contains the placeholder and both values.
    let msg = e.to_string();
    assert!(msg.contains("name"));
    assert!(msg.contains("a"));
    assert!(msg.contains("b"));
}

#[test]
fn template_trait_can_be_implemented_minimally() {
    struct S;
    impl Template for S {
        type Error = templatia::TemplateError;
        type Struct = S;
        fn to_string(&self) -> String { "x".into() }
        fn from_string(_: &str) -> Result<Self::Struct, Self::Error> { Ok(S) }
    }

    assert_eq!(S.to_string(), "x");
    let _ = S::from_string("").unwrap();
}
