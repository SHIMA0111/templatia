use templatia::{Template, TemplateError};

// Tests focusing on internal escaping of ':' used in error handling paths.
// These ensure that user-facing error values correctly contain ':' characters
// and are not polluted by any internal escape markers.

mod colon_escape_error_tests {
    use super::*;

    #[test]
    fn inconsistent_values_preserves_colons() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "first={name}, second={name}")]
        struct WithName {
            name: String,
        }

        // Different values for duplicate placeholder; both contain ':'
        let result = WithName::from_str("first=a:b, second=c:d");
        match result {
            Err(TemplateError::InconsistentValues {
                placeholder,
                first_value,
                second_value,
            }) => {
                assert_eq!(placeholder, "name");
                assert_eq!(first_value, "a:b");
                assert_eq!(second_value, "c:d");
            }
            other => panic!("Expected InconsistentValues error, got: {other:?}"),
        }
    }

    #[test]
    fn parse_to_type_value_with_colon() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "port={port}")]
        struct PortCfg {
            port: u16,
        }

        // Value contains ':' which makes number parsing fail
        let result = PortCfg::from_str("port=12:34");
        match result {
            Err(TemplateError::ParseToType {
                placeholder,
                value,
                type_name,
            }) => {
                assert_eq!(placeholder, "port");
                assert_eq!(value, "12:34");
                assert_eq!(type_name, "u16");
            }
            other => panic!("Expected ParseToType error, got: {other:?}"),
        }
    }

    #[test]
    fn unexpected_input_expected_literal_contains_colon() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "Hello {name}: world")]
        struct Greeter {
            name: String,
        }

        // After parsing "Hello " and {name}, the next literal is ": world".
        // Provide input where that literal does not match to trigger UnexpectedInput.
        let result = Greeter::from_str("Hello Alice- world");
        match result {
            Err(TemplateError::UnexpectedInput {
                expected_next_literal,
                remaining_text,
            }) => {
                // The expected literal should include ':'
                assert_eq!(expected_next_literal, ": world");
                // Remaining text starts from the last successfully matched literal (before the placeholder),
                // so it includes the placeholder value followed by the mismatching part.
                assert_eq!(remaining_text, "Alice- world");
            }
            other => panic!("Expected UnexpectedInput error, got: {other:?}"),
        }
    }

    #[test]
    fn unexpected_input_remaining_text_contains_colon() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "Hello {name} world")]
        struct Greeter2 {
            name: String,
        }

        // Here the next literal is " world". We pass remaining text that begins with ':'
        // to ensure ':' is preserved in the user-facing error.
        let result = Greeter2::from_str("Hello Alice:world");
        match result {
            Err(TemplateError::UnexpectedInput {
                expected_next_literal,
                remaining_text,
            }) => {
                assert_eq!(expected_next_literal, " world");
                // Remaining text starts from the previous literal; includes parsed placeholder value
                assert_eq!(remaining_text, "Alice:world");
            }
            other => panic!("Expected UnexpectedInput error, got: {other:?}"),
        }
    }
}
