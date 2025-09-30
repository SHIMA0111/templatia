use templatia::{Template, TemplateError};
// Tests follow AGENTS.md policy. They express intended behavior from docs.

/// Tests for default template behavior and edge cases
mod default_template_tests {
    use super::*;

    #[test]
    fn default_template_single_field() {
        #[derive(Template, Debug, PartialEq)]
        struct SingleField {
            name: String,
        }

        let single = SingleField {
            name: "test".into(),
        };
        let template = single.to_string();
        assert_eq!(template, "name = test");

        let parsed = SingleField::from_string(&template).unwrap();
        assert_eq!(parsed, single);
    }

    #[test]
    fn default_template_multiple_fields_preserves_order() {
        #[derive(Template, Debug, PartialEq)]
        struct MultiField {
            alpha: String,
            beta: u32,
            gamma: bool,
        }

        let multi = MultiField {
            alpha: "first".into(),
            beta: 42,
            gamma: true,
        };
        let template = multi.to_string();
        assert_eq!(template, "alpha = first\nbeta = 42\ngamma = true");

        let parsed = MultiField::from_string(&template).unwrap();
        assert_eq!(parsed, multi);
    }

    #[test]
    fn default_template_numeric_types() {
        #[derive(Template, Debug, PartialEq)]
        struct NumericTypes {
            byte_val: u8,
            short_val: u16,
            int_val: u32,
            long_val: u64,
            signed_val: i32,
            float_val: f32,
            double_val: f64,
        }

        let nums = NumericTypes {
            byte_val: 255,
            short_val: 65535,
            int_val: 4294967295,
            long_val: 18446744073709551615,
            signed_val: -2147483648,
            float_val: std::f32::consts::PI,
            double_val: std::f64::consts::E,
        };

        let template = nums.to_string();
        let parsed = NumericTypes::from_string(&template).unwrap();
        assert_eq!(parsed, nums);
    }

    #[test]
    fn default_template_boolean_values() {
        #[derive(Template, Debug, PartialEq)]
        struct BoolValues {
            flag_true: bool,
            flag_false: bool,
        }

        let bools = BoolValues {
            flag_true: true,
            flag_false: false,
        };

        let template = bools.to_string();
        assert_eq!(template, "flag_true = true\nflag_false = false");

        let parsed = BoolValues::from_string(&template).unwrap();
        assert_eq!(parsed, bools);
    }

    #[test]
    fn default_template_empty_string() {
        #[derive(Template, Debug, PartialEq)]
        struct EmptyString {
            empty: String,
            normal: String,
        }

        let empty_str = EmptyString {
            empty: "".into(),
            normal: "content".into(),
        };

        let template = empty_str.to_string();
        assert_eq!(template, "empty = \nnormal = content");

        let parsed = EmptyString::from_string(&template).unwrap();
        assert_eq!(parsed, empty_str);
    }
}

/// Tests for custom template behavior and edge cases
mod custom_template_tests {
    use super::*;

    #[test]
    fn custom_template_complex_format() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "Server: {host} | Port: {port} | DB: {database}")]
        struct ComplexFormat {
            host: String,
            port: u16,
            database: String,
        }

        let config = ComplexFormat {
            host: "db.example.com".into(),
            port: 5432,
            database: "production".into(),
        };

        let template = config.to_string();
        assert_eq!(template, "Server: db.example.com | Port: 5432 | DB: production");

        let parsed = ComplexFormat::from_string(&template).unwrap();
        assert_eq!(parsed, config);
    }

    #[test]
    fn custom_template_url_format() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "https://{host}:{port}/{path}?token={token}")]
        struct UrlFormat {
            host: String,
            port: u16,
            path: String,
            token: String,
        }

        let url = UrlFormat {
            host: "api.example.com".into(),
            port: 443,
            path: "v1/data".into(),
            token: "abc123".into(),
        };

        let template = url.to_string();
        assert_eq!(template, "https://api.example.com:443/v1/data?token=abc123");

        let parsed = UrlFormat::from_string(&template).unwrap();
        assert_eq!(parsed, url);
    }

    #[test]
    fn custom_template_json_like_format() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = r#"{{"name": "{name}", "age": {age}, "active": {active}}}"#)]
        struct JsonLike {
            name: String,
            age: u32,
            active: bool,
        }

        let person = JsonLike {
            name: "Alice".into(),
            age: 30,
            active: true,
        };

        let template = person.to_string();
        assert_eq!(template, r#"{"name": "Alice", "age": 30, "active": true}"#);

        let parsed = JsonLike::from_string(&template).unwrap();
        assert_eq!(parsed, person);
    }

    #[test]
    fn custom_template_with_special_characters() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "User: {name}\nEmail: {email}\nNotes: {notes}")]
        struct SpecialChars {
            name: String,
            email: String,
            notes: String,
        }

        let user = SpecialChars {
            name: "John Doe".into(),
            email: "john@example.com".into(),
            notes: "Test notes with symbols: @#$%^&*()".into(),
        };

        let template = user.to_string();
        let parsed = SpecialChars::from_string(&template).unwrap();
        assert_eq!(parsed, user);
    }

    #[test]
    fn custom_template_minimal_format() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{value}")]
        struct Minimal {
            value: String,
        }

        let minimal = Minimal {
            value: "just_this".into(),
        };

        let template = minimal.to_string();
        assert_eq!(template, "just_this");

        let parsed = Minimal::from_string(&template).unwrap();
        assert_eq!(parsed, minimal);
    }
}

/// Tests for duplicate placeholder behavior
mod duplicate_placeholder_tests {
    use super::*;

    #[test]
    fn duplicate_placeholders_consistent_values() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "Hello {name}! Welcome back, {name}!")]
        struct Greeting {
            name: String,
        }

        let greeting = Greeting {
            name: "Alice".into(),
        };

        let template = greeting.to_string();
        assert_eq!(template, "Hello Alice! Welcome back, Alice!");

        let parsed = Greeting::from_string(&template).unwrap();
        assert_eq!(parsed, greeting);
    }

    #[test]
    fn duplicate_placeholders_many_occurrences() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{id}-{id}-{id}-{id}")]
        struct MultiId {
            id: u32,
        }

        let multi = MultiId { id: 12345 };

        let template = multi.to_string();
        assert_eq!(template, "12345-12345-12345-12345");

        let parsed = MultiId::from_string(&template).unwrap();
        assert_eq!(parsed, multi);
    }

    #[test]
    fn duplicate_placeholders_inconsistent_values_error() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "first={name}, second={name}")]
        struct Inconsistent {
            name: String,
        }

        let result = Inconsistent::from_string("first=alice, second=bob");
        
        match result {
            Err(TemplateError::InconsistentValues {
                placeholder,
                first_value,
                second_value,
            }) => {
                assert_eq!(placeholder, "name");
                assert_eq!(first_value, "alice");
                assert_eq!(second_value, "bob");
            }
            other => panic!("Expected InconsistentValues error, got: {other:?}"),
        }
    }

    #[test]
    fn duplicate_placeholders_numeric_inconsistency() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "port1={port} port2={port}")]
        struct NumericInconsistent {
            port: u16,
        }

        let result = NumericInconsistent::from_string("port1=8080 port2=9090");
        
        match result {
            Err(TemplateError::InconsistentValues {
                placeholder,
                first_value,
                second_value,
            }) => {
                assert_eq!(placeholder, "port");
                assert_eq!(first_value, "8080");
                assert_eq!(second_value, "9090");
            }
            other => panic!("Expected InconsistentValues error, got: {other:?}"),
        }
    }

    #[test]
    fn duplicate_placeholders_mixed_with_unique() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{env}-{service}-{env}-{version}")]
        struct MixedDuplicates {
            env: String,
            service: String,
            version: String,
        }

        let mixed = MixedDuplicates {
            env: "prod".into(),
            service: "api".into(),
            version: "v1.0".into(),
        };

        let template = mixed.to_string();
        assert_eq!(template, "prod-api-prod-v1.0");

        let parsed = MixedDuplicates::from_string(&template).unwrap();
        assert_eq!(parsed, mixed);
    }
}

/// Tests for error handling and edge cases
mod error_handling_tests {
    use super::*;

    #[test]
    fn parse_error_invalid_number() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "port={port}")]
        struct PortConfig {
            port: u16,
        }

        let result = PortConfig::from_string("port=not_a_number");
        
        match result {
            Err(TemplateError::Parse(msg)) => {
                assert!(msg.contains("Failed to parse field \"port\""));
            }
            other => panic!("Expected Parse error, got: {other:?}"),
        }
    }

    #[test]
    fn parse_error_invalid_boolean() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "enabled={enabled}")]
        struct BoolConfig {
            enabled: bool,
        }

        let result = BoolConfig::from_string("enabled=maybe");
        
        match result {
            Err(TemplateError::Parse(msg)) => {
                assert!(msg.contains("Failed to parse field \"enabled\""));
            }
            other => panic!("Expected Parse error, got: {other:?}"),
        }
    }

    #[test]
    fn parse_error_number_overflow() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "value={value}")]
        struct OverflowTest {
            value: u8,
        }

        let result = OverflowTest::from_string("value=256");
        
        match result {
            Err(TemplateError::Parse(_)) => {
                // Expected - overflow should cause parse error
            }
            other => panic!("Expected Parse error for overflow, got: {other:?}"),
        }
    }

    #[test]
    fn parse_error_negative_unsigned() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "count={count}")]
        struct UnsignedTest {
            count: u32,
        }

        let result = UnsignedTest::from_string("count=-1");
        
        match result {
            Err(TemplateError::Parse(_)) => {
                // Expected - negative value for unsigned type should fail
            }
            other => panic!("Expected Parse error for negative unsigned, got: {other:?}"),
        }
    }

    #[test]
    fn parse_error_malformed_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "host={host}:{port}")]
        struct HostPort {
            host: String,
            port: u16,
        }

        // Missing colon separator
        let result = HostPort::from_string("host=localhost 8080");
        
        match result {
            Err(TemplateError::Parse(_)) => {
                // Expected - template doesn't match expected format
            }
            other => panic!("Expected Parse error for malformed template, got: {other:?}"),
        }
    }

    #[test]
    fn parse_error_partial_template_match() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "prefix_{value}_suffix")]
        struct PrefixSuffix {
            value: String,
        }

        // Missing suffix
        let result = PrefixSuffix::from_string("prefix_test");
        
        match result {
            Err(TemplateError::Parse(_)) => {
                // Expected - incomplete template match
            }
            other => panic!("Expected Parse error for partial match, got: {other:?}"),
        }
    }
}

/// Tests for type constraint edge cases
mod type_constraint_tests {
    use super::*;

    #[test]
    fn floating_point_precision() {
        #[derive(Template, Debug)]
        #[templatia(template = "value={value}")]
        struct FloatTest {
            value: f64,
        }

        let original = FloatTest { value: std::f64::consts::PI };
        let template = original.to_string();
        let parsed = FloatTest::from_string(&template).unwrap();
        
        // Allow for floating point precision differences
        assert!((parsed.value - original.value).abs() < 1e-10);
    }

    #[test]
    fn extreme_numeric_values() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "max={max_val}, min={min_val}")]
        struct ExtremeValues {
            max_val: i64,
            min_val: i64,
        }

        let extreme = ExtremeValues {
            max_val: i64::MAX,
            min_val: i64::MIN,
        };

        let template = extreme.to_string();
        let parsed = ExtremeValues::from_string(&template).unwrap();
        assert_eq!(parsed, extreme);
    }

    #[test]
    fn zero_values() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "int={int_zero}, float={float_zero}")]
        struct ZeroValues {
            int_zero: i32,
            float_zero: f64,
        }

        let zeros = ZeroValues {
            int_zero: 0,
            float_zero: 0.0,
        };

        let template = zeros.to_string();
        let parsed = ZeroValues::from_string(&template).unwrap();
        assert_eq!(parsed, zeros);
    }

    #[test]
    fn string_with_whitespace() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "name={name}")]
        struct WhitespaceTest {
            name: String,
        }

        let whitespace = WhitespaceTest {
            name: "  spaced  name  ".into(),
        };

        let template = whitespace.to_string();
        let parsed = WhitespaceTest::from_string(&template).unwrap();
        assert_eq!(parsed, whitespace);
    }

    #[test]
    fn string_with_newlines() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "content={content}")]
        struct NewlineTest {
            content: String,
        }

        let multiline = NewlineTest {
            content: "line1\nline2\nline3".into(),
        };

        let template = multiline.to_string();
        let parsed = NewlineTest::from_string(&template).unwrap();
        assert_eq!(parsed, multiline);
    }
}

/// Tests for field ordering and combinations
mod field_combination_tests {
    use super::*;

    #[test]
    fn many_fields_default_template() {
        #[derive(Template, Debug, PartialEq)]
        struct ManyFields {
            field_a: String,
            field_b: u32,
            field_c: bool,
            field_d: f64,
            field_e: i16,
            field_f: String,
        }

        let many = ManyFields {
            field_a: "alpha".into(),
            field_b: 42,
            field_c: true,
            field_d: 3.14,
            field_e: -123,
            field_f: "omega".into(),
        };

        let template = many.to_string();
        let parsed = ManyFields::from_string(&template).unwrap();
        assert_eq!(parsed, many);
    }

    #[test]
    fn mixed_field_order_in_custom_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{c}-{a}-{b}")]
        struct MixedOrder {
            a: String,
            b: u32,
            c: bool,
        }

        let mixed = MixedOrder {
            a: "middle".into(),
            b: 999,
            c: false,
        };

        let template = mixed.to_string();
        assert_eq!(template, "false-middle-999");

        let parsed = MixedOrder::from_string(&template).unwrap();
        assert_eq!(parsed, mixed);
    }

    #[test]
    fn single_field_repeated_many_times() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{x}{x}{x}{x}{x}{x}{x}{x}")]
        struct RepeatedField {
            x: char,
        }

        let repeated = RepeatedField { x: 'A' };

        let template = repeated.to_string();
        assert_eq!(template, "AAAAAAAA");

        let parsed = RepeatedField::from_string(&template).unwrap();
        assert_eq!(parsed, repeated);
    }
}

/// Tests for round-trip consistency
mod roundtrip_tests {
    use super::*;

    #[test]
    fn roundtrip_consistency_default_template() {
        #[derive(Template, Debug, PartialEq)]
        struct RoundtripTest {
            name: String,
            count: u32,
            enabled: bool,
        }

        let original = RoundtripTest {
            name: "test_data".into(),
            count: 100,
            enabled: true,
        };

        // First round-trip
        let template1 = original.to_string();
        let parsed1 = RoundtripTest::from_string(&template1).unwrap();
        assert_eq!(parsed1, original);

        // Second round-trip
        let template2 = parsed1.to_string();
        let parsed2 = RoundtripTest::from_string(&template2).unwrap();
        assert_eq!(parsed2, original);

        // Templates should be identical
        assert_eq!(template1, template2);
    }

    #[test]
    fn roundtrip_consistency_custom_template() {
        #[derive(Template, Debug, PartialEq, Clone)]
        #[templatia(template = "Config[{name}]={value}")]
        struct CustomRoundtrip {
            name: String,
            value: String,
        }

        let original = CustomRoundtrip {
            name: "database_url".into(),
            value: "postgres://localhost:5432/mydb".into(),
        };

        // Multiple round-trips
        let mut current = original.clone();
        for _ in 0..5 {
            let template = current.to_string();
            current = CustomRoundtrip::from_string(&template).unwrap();
        }

        assert_eq!(current, original);
    }

    #[test]
    fn roundtrip_with_edge_case_values() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "'{text}' #{number}")]
        struct EdgeCaseRoundtrip {
            text: String,
            number: i32,
        }

        let edge_cases = vec![
            EdgeCaseRoundtrip {
                text: "".into(),
                number: 0,
            },
            EdgeCaseRoundtrip {
                text: "single".into(),
                number: -1,
            },
            EdgeCaseRoundtrip {
                text: "with spaces and symbols: @#$%".into(),
                number: i32::MAX,
            },
            EdgeCaseRoundtrip {
                text: "unicode: 🦀 Rust 🚀".into(),
                number: i32::MIN,
            },
        ];

        for original in edge_cases {
            let template = original.to_string();
            let parsed = EdgeCaseRoundtrip::from_string(&template).unwrap();
            assert_eq!(parsed, original, "Failed roundtrip for: {:?}", original);
        }
    }
}

/// Tests for consecutive placeholder validation and new functionality
mod consecutive_placeholder_tests {
    use super::*;

    #[test]
    fn consecutive_char_placeholders_allowed() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{first}{second}")]
        struct ConsecutiveChars {
            first: char,
            second: char,
        }

        let chars = ConsecutiveChars {
            first: 'A',
            second: 'B',
        };

        let template = chars.to_string();
        assert_eq!(template, "AB");

        let parsed = ConsecutiveChars::from_string(&template).unwrap();
        assert_eq!(parsed, chars);
    }

    #[test]
    fn consecutive_bool_placeholders_allowed() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{flag1}{flag2}")]
        struct ConsecutiveBools {
            flag1: bool,
            flag2: bool,
        }

        let bools = ConsecutiveBools {
            flag1: true,
            flag2: false,
        };

        let template = bools.to_string();
        assert_eq!(template, "truefalse");

        let parsed = ConsecutiveBools::from_string(&template).unwrap();
        assert_eq!(parsed, bools);
    }

    #[test]
    fn mixed_char_bool_consecutive_allowed() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{character}{flag}")]
        struct MixedCharBool {
            character: char,
            flag: bool,
        }

        let mixed = MixedCharBool {
            character: 'X',
            flag: true,
        };

        let template = mixed.to_string();
        assert_eq!(template, "Xtrue");

        let parsed = MixedCharBool::from_string(&template).unwrap();
        assert_eq!(parsed, mixed);
    }

    #[test]
    fn bool_char_consecutive_allowed() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{enabled}{grade}")]
        struct BoolChar {
            enabled: bool,
            grade: char,
        }

        let bc = BoolChar {
            enabled: false,
            grade: 'A',
        };

        let template = bc.to_string();
        assert_eq!(template, "falseA");

        let parsed = BoolChar::from_string(&template).unwrap();
        assert_eq!(parsed, bc);
    }

    #[test]
    fn multiple_consecutive_chars() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{a}{b}{c}{d}")]
        struct MultipleChars {
            a: char,
            b: char,
            c: char,
            d: char,
        }

        let multi = MultipleChars {
            a: 'T',
            b: 'E',
            c: 'S',
            d: 'T',
        };

        let template = multi.to_string();
        assert_eq!(template, "TEST");

        let parsed = MultipleChars::from_string(&template).unwrap();
        assert_eq!(parsed, multi);
    }

    #[test]
    fn multiple_consecutive_bools() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{a}{b}{c}")]
        struct MultipleBools {
            a: bool,
            b: bool,
            c: bool,
        }

        let multi = MultipleBools {
            a: true,
            b: false,
            c: true,
        };

        let template = multi.to_string();
        assert_eq!(template, "truefalsetrue");

        let parsed = MultipleBools::from_string(&template).unwrap();
        assert_eq!(parsed, multi);
    }

    #[test]
    fn char_with_literal_separator() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{first}-{second}")]
        struct SeparatedChars {
            first: char,
            second: char,
        }

        let sep = SeparatedChars {
            first: 'A',
            second: 'Z',
        };

        let template = sep.to_string();
        assert_eq!(template, "A-Z");

        let parsed = SeparatedChars::from_string(&template).unwrap();
        assert_eq!(parsed, sep);
    }

    #[test]
    fn bool_with_literal_separator() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{enabled}|{disabled}")]
        struct SeparatedBools {
            enabled: bool,
            disabled: bool,
        }

        let sep = SeparatedBools {
            enabled: true,
            disabled: false,
        };

        let template = sep.to_string();
        assert_eq!(template, "true|false");

        let parsed = SeparatedBools::from_string(&template).unwrap();
        assert_eq!(parsed, sep);
    }
}

/// Tests for escaped brace handling
mod escaped_brace_tests {
    use super::*;

    #[test]
    fn escaped_opening_brace() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{{value={value}")]
        struct EscapedOpen {
            value: String,
        }

        let escaped = EscapedOpen {
            value: "test".into(),
        };

        let template = escaped.to_string();
        assert_eq!(template, "{value=test");

        let parsed = EscapedOpen::from_string(&template).unwrap();
        assert_eq!(parsed, escaped);
    }

    #[test]
    fn escaped_closing_brace() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "value={value}}}")]
        struct EscapedClose {
            value: String,
        }

        let escaped = EscapedClose {
            value: "test".into(),
        };

        let template = escaped.to_string();
        assert_eq!(template, "value=test}");

        let parsed = EscapedClose::from_string(&template).unwrap();
        assert_eq!(parsed, escaped);
    }

    #[test]
    fn both_escaped_braces() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{{data: {value}}}")]
        struct BothEscaped {
            value: String,
        }

        let escaped = BothEscaped {
            value: "test".into(),
        };

        let template = escaped.to_string();
        assert_eq!(template, "{data: test}");

        let parsed = BothEscaped::from_string(&template).unwrap();
        assert_eq!(parsed, escaped);
    }

    #[test]
    fn multiple_escaped_braces() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{{{{prefix}}}}: {value}")]
        struct MultipleEscaped {
            value: String,
        }

        let escaped = MultipleEscaped {
            value: "data".into(),
        };

        let template = escaped.to_string();
        assert_eq!(template, "{{prefix}}: data");

        let parsed = MultipleEscaped::from_string(&template).unwrap();
        assert_eq!(parsed, escaped);
    }
}

/// Tests for missing field support (allow_missing_placeholders attribute)
mod missing_field_tests {
    use super::*;

    #[test]
    fn basic_missing_field_with_default() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "name={name}", allow_missing_placeholders)]
        struct PartialConfig {
            name: String,
            port: u16,
        }

        // Create instance with both fields
        let config = PartialConfig {
            name: "server".into(),
            port: 8080,
        };

        // to_string only includes template fields
        let template = config.to_string();
        assert_eq!(template, "name=server");

        // from_string sets missing field to default
        let parsed = PartialConfig::from_string(&template).unwrap();
        assert_eq!(parsed.name, "server");
        assert_eq!(parsed.port, 0); // Default for u16
    }

    #[test]
    fn multiple_missing_fields() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "id={id}", allow_missing_placeholders)]
        struct MultiMissing {
            id: u32,
            name: String,
            enabled: bool,
            count: i32,
        }

        let instance = MultiMissing {
            id: 42,
            name: "test".into(),
            enabled: true,
            count: -5,
        };

        let template = instance.to_string();
        assert_eq!(template, "id=42");

        let parsed = MultiMissing::from_string(&template).unwrap();
        assert_eq!(parsed.id, 42);
        assert_eq!(parsed.name, ""); // Default for String
        assert_eq!(parsed.enabled, false); // Default for bool
        assert_eq!(parsed.count, 0); // Default for i32
    }

    #[test]
    fn missing_field_with_some_in_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "user={user}:pass={pass}", allow_missing_placeholders)]
        struct Credentials {
            user: String,
            pass: String,
            domain: String,
            port: u16,
        }

        let creds = Credentials {
            user: "admin".into(),
            pass: "secret".into(),
            domain: "example.com".into(),
            port: 443,
        };

        let template = creds.to_string();
        assert_eq!(template, "user=admin:pass=secret");

        let parsed = Credentials::from_string(&template).unwrap();
        assert_eq!(parsed.user, "admin");
        assert_eq!(parsed.pass, "secret");
        assert_eq!(parsed.domain, ""); // Default
        assert_eq!(parsed.port, 0); // Default
    }

    #[test]
    fn missing_field_all_types_have_defaults() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "active={active}", allow_missing_placeholders)]
        struct AllTypes {
            active: bool,
            text: String,
            number: i32,
            unsigned: u64,
            float: f64,
            character: char,
        }

        let instance = AllTypes {
            active: true,
            text: "ignored".into(),
            number: 999,
            unsigned: 12345,
            float: 3.14,
            character: 'X',
        };

        let template = instance.to_string();
        assert_eq!(template, "active=true");

        let parsed = AllTypes::from_string(&template).unwrap();
        assert_eq!(parsed.active, true);
        assert_eq!(parsed.text, "");
        assert_eq!(parsed.number, 0);
        assert_eq!(parsed.unsigned, 0);
        assert_eq!(parsed.float, 0.0);
        assert_eq!(parsed.character, '\0');
    }

    #[test]
    fn missing_field_roundtrip_consistency() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "status={status}", allow_missing_placeholders)]
        struct StatusReport {
            status: String,
            timestamp: u64,
            level: u8,
        }

        // First roundtrip - with values
        let original = StatusReport {
            status: "ok".into(),
            timestamp: 1234567890,
            level: 5,
        };

        let template1 = original.to_string();
        let parsed1 = StatusReport::from_string(&template1).unwrap();
        
        // Missing fields get defaults
        assert_eq!(parsed1.status, "ok");
        assert_eq!(parsed1.timestamp, 0);
        assert_eq!(parsed1.level, 0);

        // Second roundtrip - template stays the same
        let template2 = parsed1.to_string();
        assert_eq!(template1, template2);
        
        let parsed2 = StatusReport::from_string(&template2).unwrap();
        assert_eq!(parsed2, parsed1);
    }

    #[test]
    fn missing_field_with_duplicate_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{id}-{id}-{id}", allow_missing_placeholders)]
        struct DuplicateWithMissing {
            id: char,
            extra: String,
        }

        let instance = DuplicateWithMissing {
            id: 'A',
            extra: "not_in_template".into(),
        };

        let template = instance.to_string();
        assert_eq!(template, "A-A-A");

        let parsed = DuplicateWithMissing::from_string(&template).unwrap();
        assert_eq!(parsed.id, 'A');
        assert_eq!(parsed.extra, ""); // Default
    }

    #[test]
    fn missing_field_empty_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "constant_text", allow_missing_placeholders)]
        struct NoPlaceholders {
            field1: String,
            field2: u32,
        }

        let instance = NoPlaceholders {
            field1: "value".into(),
            field2: 100,
        };

        let template = instance.to_string();
        assert_eq!(template, "constant_text");

        let parsed = NoPlaceholders::from_string(&template).unwrap();
        assert_eq!(parsed.field1, "");
        assert_eq!(parsed.field2, 0);
    }

    #[test]
    fn missing_field_complex_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "Name: {name}, Age: {age}", allow_missing_placeholders)]
        struct Person {
            name: String,
            age: u8,
            email: String,
            phone: String,
            address: String,
        }

        let person = Person {
            name: "Alice".into(),
            age: 30,
            email: "alice@example.com".into(),
            phone: "555-1234".into(),
            address: "123 Main St".into(),
        };

        let template = person.to_string();
        assert_eq!(template, "Name: Alice, Age: 30");

        let parsed = Person::from_string(&template).unwrap();
        assert_eq!(parsed.name, "Alice");
        assert_eq!(parsed.age, 30);
        assert_eq!(parsed.email, "");
        assert_eq!(parsed.phone, "");
        assert_eq!(parsed.address, "");
    }

    #[test]
    fn missing_field_with_consecutive_types() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{flag}{ch}", allow_missing_placeholders)]
        struct ConsecutiveWithMissing {
            flag: bool,
            ch: char,
            extra1: String,
            extra2: u32,
        }

        let instance = ConsecutiveWithMissing {
            flag: true,
            ch: 'X',
            extra1: "ignored".into(),
            extra2: 999,
        };

        let template = instance.to_string();
        assert_eq!(template, "trueX");

        let parsed = ConsecutiveWithMissing::from_string(&template).unwrap();
        assert_eq!(parsed.flag, true);
        assert_eq!(parsed.ch, 'X');
        assert_eq!(parsed.extra1, "");
        assert_eq!(parsed.extra2, 0);
    }

    #[test]
    fn missing_field_only_first_field_in_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{first}", allow_missing_placeholders)]
        struct FirstOnly {
            first: String,
            second: String,
            third: String,
        }

        let instance = FirstOnly {
            first: "one".into(),
            second: "two".into(),
            third: "three".into(),
        };

        let template = instance.to_string();
        assert_eq!(template, "one");

        let parsed = FirstOnly::from_string(&template).unwrap();
        assert_eq!(parsed.first, "one");
        assert_eq!(parsed.second, "");
        assert_eq!(parsed.third, "");
    }

    #[test]
    fn missing_field_only_last_field_in_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "result={last}", allow_missing_placeholders)]
        struct LastOnly {
            first: u32,
            second: bool,
            last: String,
        }

        let instance = LastOnly {
            first: 100,
            second: true,
            last: "final".into(),
        };

        let template = instance.to_string();
        assert_eq!(template, "result=final");

        let parsed = LastOnly::from_string(&template).unwrap();
        assert_eq!(parsed.first, 0);
        assert_eq!(parsed.second, false);
        assert_eq!(parsed.last, "final");
    }

    #[test]
    fn missing_field_alternating_pattern() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{a}:{c}", allow_missing_placeholders)]
        struct Alternating {
            a: String,
            b: String,
            c: String,
            d: String,
        }

        let instance = Alternating {
            a: "A".into(),
            b: "B".into(),
            c: "C".into(),
            d: "D".into(),
        };

        let template = instance.to_string();
        assert_eq!(template, "A:C");

        let parsed = Alternating::from_string(&template).unwrap();
        assert_eq!(parsed.a, "A");
        assert_eq!(parsed.b, ""); // Missing
        assert_eq!(parsed.c, "C");
        assert_eq!(parsed.d, ""); // Missing
    }
}