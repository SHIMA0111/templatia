use templatia::{Template, TemplateError};
// Tests for Option<T> support - follows AGENTS.md policy

/// Tests for basic Option<T> field behavior
mod basic_option_tests {
    use super::*;


    #[test]
    fn option_field_present_in_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "name={name}")]
        struct OptionalName {
            name: Option<String>,
        }

        let with_name = OptionalName {
            name: Some("Alice".into()),
        };

        let template = with_name.render_string();
        assert_eq!(template, "name=Alice");

        let parsed = OptionalName::from_str(&template).unwrap();
        assert_eq!(parsed, with_name);
        assert_eq!(parsed.name, Some("Alice".into()));
    }

    #[test]
    fn option_field_missing_from_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "id={id}", allow_missing_placeholders)]
        struct OptionalFields {
            id: u32,
            name: Option<String>,
        }

        let instance = OptionalFields {
            id: 42,
            name: Some("test".into()),
        };

        let template = instance.render_string();
        assert_eq!(template, "id=42");

        let parsed = OptionalFields::from_str(&template).unwrap();
        assert_eq!(parsed.id, 42);
        assert_eq!(parsed.name, None); // Optional field not in template should be None
    }

    #[test]
    fn option_field_vs_required_field_missing() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "id={id}", allow_missing_placeholders)]
        struct MixedFields {
            id: u32,
            name: String,
            email: Option<String>,
        }

        let instance = MixedFields {
            id: 100,
            name: "default".into(),
            email: Some("test@example.com".into()),
        };

        let template = instance.render_string();
        let parsed = MixedFields::from_str(&template).unwrap();

        assert_eq!(parsed.id, 100);
        assert_eq!(parsed.name, ""); // Non-optional gets Default
        assert_eq!(parsed.email, None); // Optional gets None
    }

    #[test]
    fn multiple_option_fields_some_present() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "name={name}", allow_missing_placeholders)]
        struct MultipleOptional {
            name: Option<String>,
            age: Option<u32>,
            email: Option<String>,
        }

        let instance = MultipleOptional {
            name: Some("Bob".into()),
            age: Some(25),
            email: Some("bob@example.com".into()),
        };

        let template = instance.render_string();
        assert_eq!(template, "name=Bob");

        let parsed = MultipleOptional::from_str(&template).unwrap();
        assert_eq!(parsed.name, Some("Bob".into()));
        assert_eq!(parsed.age, None);
        assert_eq!(parsed.email, None);
    }

    #[test]
    fn all_option_fields_in_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "name={name}, age={age}")]
        struct AllPresent {
            name: Option<String>,
            age: Option<u32>,
        }

        let instance = AllPresent {
            name: Some("Charlie".into()),
            age: Some(30),
        };

        let template = instance.render_string();
        assert_eq!(template, "name=Charlie, age=30");

        let parsed = AllPresent::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
        assert_eq!(parsed.name, Some("Charlie".into()));
        assert_eq!(parsed.age, Some(30));
    }

    #[test]
    fn option_field_none_in_struct_but_present_in_template() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "value={value}")]
        struct OptionalValue {
            value: Option<String>,
        }

        // When serializing None, it should still produce the template format
        let with_none = OptionalValue { value: None };
        let _template = with_none.render_string();
        // The None value will be serialized as empty string or similar
        
        // But parsing from a valid template should give Some
        let parsed = OptionalValue::from_str("value=test").unwrap();
        assert_eq!(parsed.value, Some("test".into()));
    }
}

/// Tests for Option<T> with various types
mod option_type_tests {
    use super::*;

    #[test]
    fn option_string_type() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "text={text}")]
        struct OptionalString {
            text: Option<String>,
        }

        let instance = OptionalString {
            text: Some("hello world".into()),
        };

        let template = instance.render_string();
        let parsed = OptionalString::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn option_numeric_types() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "u8={u8_val}, u16={u16_val}, u32={u32_val}, i32={i32_val}")]
        struct OptionalNumerics {
            u8_val: Option<u8>,
            u16_val: Option<u16>,
            u32_val: Option<u32>,
            i32_val: Option<i32>,
        }

        let instance = OptionalNumerics {
            u8_val: Some(255),
            u16_val: Some(65535),
            u32_val: Some(4294967295),
            i32_val: Some(-2147483648),
        };

        let template = instance.render_string();
        let parsed = OptionalNumerics::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn option_bool_type() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "flag={flag}")]
        struct OptionalBool {
            flag: Option<bool>,
        }

        let with_true = OptionalBool { flag: Some(true) };
        let template_true = with_true.render_string();
        assert_eq!(template_true, "flag=true");

        let parsed_true = OptionalBool::from_str(&template_true).unwrap();
        assert_eq!(parsed_true, with_true);

        let with_false = OptionalBool { flag: Some(false) };
        let template_false = with_false.render_string();
        assert_eq!(template_false, "flag=false");

        let parsed_false = OptionalBool::from_str(&template_false).unwrap();
        assert_eq!(parsed_false, with_false);
    }

    #[test]
    fn option_char_type() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "ch={ch}")]
        struct OptionalChar {
            ch: Option<char>,
        }

        let instance = OptionalChar { ch: Some('X') };
        let template = instance.render_string();
        assert_eq!(template, "ch=X");

        let parsed = OptionalChar::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn option_float_types() {
        #[derive(Template, Debug)]
        #[templatia(template = "f32={f32_val}, f64={f64_val}")]
        struct OptionalFloats {
            f32_val: Option<f32>,
            f64_val: Option<f64>,
        }

        let instance = OptionalFloats {
            f32_val: Some(3.14),
            f64_val: Some(std::f64::consts::E),
        };

        let template = instance.render_string();
        let parsed = OptionalFloats::from_str(&template).unwrap();
        
        assert!((parsed.f32_val.unwrap() - 3.14).abs() < 1e-5);
        assert!((parsed.f64_val.unwrap() - std::f64::consts::E).abs() < 1e-10);
    }
}

/// Tests for Option<T> with complex templates
mod option_complex_tests {
    use super::*;

    #[test]
    fn option_with_duplicate_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "first={id}, second={id}")]
        struct DuplicateOptional {
            id: Option<String>,
        }

        let instance = DuplicateOptional {
            id: Some("value".into()),
        };

        let template = instance.render_string();
        assert_eq!(template, "first=value, second=value");

        let parsed = DuplicateOptional::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn option_with_duplicate_inconsistent_values_error() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "a={val}, b={val}")]
        struct DuplicateCheck {
            val: Option<String>,
        }

        let result = DuplicateCheck::from_str("a=first, b=second");
        
        match result {
            Err(TemplateError::InconsistentValues {
                placeholder,
                first_value,
                second_value,
            }) => {
                assert_eq!(placeholder, "val");
                assert_eq!(first_value, "first");
                assert_eq!(second_value, "second");
            }
            other => panic!("Expected InconsistentValues error, got: {other:?}"),
        }
    }

    #[test]
    fn option_with_consecutive_char_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{first}{second}")]
        struct ConsecutiveOptionalChars {
            first: Option<char>,
            second: Option<char>,
        }

        let instance = ConsecutiveOptionalChars {
            first: Some('A'),
            second: Some('B'),
        };

        let template = instance.render_string();
        assert_eq!(template, "AB");

        let parsed = ConsecutiveOptionalChars::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn option_with_consecutive_bool_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{flag1}{flag2}")]
        struct ConsecutiveOptionalBools {
            flag1: Option<bool>,
            flag2: Option<bool>,
        }

        let instance = ConsecutiveOptionalBools {
            flag1: Some(true),
            flag2: Some(false),
        };

        let template = instance.render_string();
        assert_eq!(template, "truefalse");

        let parsed = ConsecutiveOptionalBools::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn option_url_format() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "https://{host}:{port}/{path}", allow_missing_placeholders)]
        struct OptionalUrl {
            host: String,
            port: Option<u16>,
            path: Option<String>,
        }

        let full_url = OptionalUrl {
            host: "example.com".into(),
            port: Some(8080),
            path: Some("api/v1".into()),
        };

        let template = full_url.render_string();
        assert_eq!(template, "https://example.com:8080/api/v1");

        let parsed = OptionalUrl::from_str(&template).unwrap();
        assert_eq!(parsed, full_url);

        // Test with only required field
        let minimal_template = "https://example.com:80/";
        let parsed_minimal = OptionalUrl::from_str(minimal_template).unwrap();
        assert_eq!(parsed_minimal.host, "example.com");
        assert_eq!(parsed_minimal.port, Some(80));
        // New behavior: empty string path is parsed as None
        assert_eq!(parsed_minimal.path, None);
    }

    #[test]
    fn option_json_like_format() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = r#"{{"name": "{name}", "age": {age}}}"#, allow_missing_placeholders)]
        struct OptionalPerson {
            name: String,
            age: Option<u32>,
            email: Option<String>,
        }

        let person = OptionalPerson {
            name: "Alice".into(),
            age: Some(30),
            email: Some("alice@example.com".into()),
        };

        let template = person.render_string();
        assert_eq!(template, r#"{"name": "Alice", "age": 30}"#);

        let parsed = OptionalPerson::from_str(&template).unwrap();
        assert_eq!(parsed.name, "Alice");
        assert_eq!(parsed.age, Some(30));
        assert_eq!(parsed.email, None); // Not in template
    }
}

/// Tests for Option<T> roundtrip behavior
mod option_roundtrip_tests {
    use super::*;

    #[test]
    fn option_roundtrip_with_value() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "data={data}")]
        struct SimpleOptional {
            data: Option<String>,
        }

        let original = SimpleOptional {
            data: Some("test_value".into()),
        };

        let template1 = original.render_string();
        let parsed1 = SimpleOptional::from_str(&template1).unwrap();
        assert_eq!(parsed1, original);

        let template2 = parsed1.render_string();
        let parsed2 = SimpleOptional::from_str(&template2).unwrap();
        assert_eq!(parsed2, original);

        assert_eq!(template1, template2);
    }

    #[test]
    fn option_roundtrip_missing_field() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "id={id}", allow_missing_placeholders)]
        struct WithOptional {
            id: u32,
            optional_field: Option<String>,
        }

        let original = WithOptional {
            id: 42,
            optional_field: Some("ignored".into()),
        };

        let template1 = original.render_string();
        let parsed1 = WithOptional::from_str(&template1).unwrap();
        
        assert_eq!(parsed1.id, 42);
        assert_eq!(parsed1.optional_field, None);

        let template2 = parsed1.render_string();
        assert_eq!(template1, template2);

        let parsed2 = WithOptional::from_str(&template2).unwrap();
        assert_eq!(parsed2, parsed1);
    }

    #[test]
    fn option_multiple_roundtrips() {
        #[derive(Template, Debug, PartialEq, Clone)]
        #[templatia(template = "name={name}, count={count}")]
        struct MultiOptional {
            name: Option<String>,
            count: Option<u32>,
        }

        let original = MultiOptional {
            name: Some("test".into()),
            count: Some(100),
        };

        let mut current = original.clone();
        for _ in 0..5 {
            let template = current.render_string();
            current = MultiOptional::from_str(&template).unwrap();
        }

        assert_eq!(current, original);
    }
}

/// Tests for Option<T> error handling
mod option_error_tests {
    use super::*;

    #[test]
    fn option_parse_error_invalid_type() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "port={port}")]
        struct OptionalPort {
            port: Option<u16>,
        }

        // New behavior: invalid values for Option<T> become None instead of error
        let result = OptionalPort::from_str("port=invalid");
        match result {
            Ok(_) => panic!("Expected Parse error, got Ok"),
            Err(TemplateError::Parse(msg)) => {
                assert!(msg.contains("Failed to parse field \"port\""));
            }
            other => panic!("Expected Parse error, got: {other:?}"),
        }
    }

    #[test]
    fn option_bool_invalid_value() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "flag={flag}")]
        struct OptionalFlag {
            flag: Option<bool>,
        }

        let result = OptionalFlag::from_str("flag=maybe");
        
        match result {
            Err(TemplateError::Parse(msg)) => {
                assert!(msg.contains("Failed to parse field \"flag\""));
            }
            other => panic!("Expected Parse error, got: {other:?}"),
        }
    }
}

/// Tests for Option<T> with mixed field types
mod option_mixed_tests {
    use super::*;

    #[test]
    fn mixed_optional_and_required_fields() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "id={id}, name={name}", allow_missing_placeholders)]
        struct MixedConfig {
            id: u32,
            name: String,
            optional_email: Option<String>,
            optional_age: Option<u8>,
        }

        let config = MixedConfig {
            id: 1,
            name: "Test".into(),
            optional_email: Some("test@example.com".into()),
            optional_age: Some(25),
        };

        let template = config.render_string();
        assert_eq!(template, "id=1, name=Test");

        let parsed = MixedConfig::from_str(&template).unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, "Test");
        assert_eq!(parsed.optional_email, None);
        assert_eq!(parsed.optional_age, None);
    }

    #[test]
    fn all_optional_fields() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "a={a}", allow_missing_placeholders)]
        struct AllOptional {
            a: Option<String>,
            b: Option<u32>,
            c: Option<bool>,
        }

        let instance = AllOptional {
            a: Some("value".into()),
            b: Some(42),
            c: Some(true),
        };

        let template = instance.render_string();
        let parsed = AllOptional::from_str(&template).unwrap();
        
        assert_eq!(parsed.a, Some("value".into()));
        assert_eq!(parsed.b, None);
        assert_eq!(parsed.c, None);
    }

    #[test]
    fn optional_with_default_template() {
        #[derive(Template, Debug, PartialEq)]
        struct DefaultWithOptional {
            required: String,
            optional: Option<String>,
        }

        let instance = DefaultWithOptional {
            required: "must_have".into(),
            optional: Some("maybe".into()),
        };

        let template = instance.render_string();
        assert_eq!(template, "required = must_have\noptional = maybe");

        let parsed = DefaultWithOptional::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn optional_fields_in_various_positions() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{a}-{b}-{c}", allow_missing_placeholders)]
        struct VariousPositions {
            a: Option<String>,
            b: String,
            c: Option<u32>,
            d: String,
        }

        let instance = VariousPositions {
            a: Some("A".into()),
            b: "B".into(),
            c: Some(3),
            d: "D".into(),
        };

        let template = instance.render_string();
        assert_eq!(template, "A-B-3");

        let parsed = VariousPositions::from_str(&template).unwrap();
        assert_eq!(parsed.a, Some("A".into()));
        assert_eq!(parsed.b, "B");
        assert_eq!(parsed.c, Some(3));
        assert_eq!(parsed.d, ""); // Default for String
    }

    #[test]
    fn option_with_empty_string_value() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "prefix={value}")]
        struct EmptyOptional {
            value: Option<String>,
        }

        let template = "prefix=";
        let parsed = EmptyOptional::from_str(template).unwrap();
        // New behavior: empty string is parsed as None by default
        assert_eq!(parsed.value, None);
    }

    #[test]
    fn complex_scenario_with_options() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(
            template = "Server: {host}:{port} | DB: {database}",
            allow_missing_placeholders
        )]
        struct ServerConfig {
            host: String,
            port: u16,
            database: String,
            username: Option<String>,
            password: Option<String>,
            ssl: Option<bool>,
            timeout: Option<u32>,
        }

        let config = ServerConfig {
            host: "localhost".into(),
            port: 5432,
            database: "mydb".into(),
            username: Some("admin".into()),
            password: Some("secret".into()),
            ssl: Some(true),
            timeout: Some(30),
        };

        let template = config.render_string();
        assert_eq!(template, "Server: localhost:5432 | DB: mydb");

        let parsed = ServerConfig::from_str(&template).unwrap();
        assert_eq!(parsed.host, "localhost");
        assert_eq!(parsed.port, 5432);
        assert_eq!(parsed.database, "mydb");
        assert_eq!(parsed.username, None);
        assert_eq!(parsed.password, None);
        assert_eq!(parsed.ssl, None);
        assert_eq!(parsed.timeout, None);
    }
}

/// Tests for empty_str_option_not_none attribute
mod empty_str_option_not_none_tests {
    use super::*;

    #[test]
    fn empty_string_as_some_with_attribute() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "prefix={value}", empty_str_option_not_none)]
        struct EmptyAsIs {
            value: Option<String>,
        }

        let template = "prefix=";
        let parsed = EmptyAsIs::from_str(template).unwrap();
        assert_eq!(parsed.value, Some("".into()));
    }

    #[test]
    fn empty_string_default_behavior() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "prefix={value}")]
        struct EmptyAsNone {
            value: Option<String>,
        }

        let template = "prefix=";
        let parsed = EmptyAsNone::from_str(template).unwrap();
        assert_eq!(parsed.value, None);
    }

    #[test]
    fn non_empty_string_with_attribute() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "prefix={value}", empty_str_option_not_none)]
        struct NonEmpty {
            value: Option<String>,
        }

        let template = "prefix=test";
        let parsed = NonEmpty::from_str(template).unwrap();
        assert_eq!(parsed.value, Some("test".into()));
    }

    #[test]
    fn multiple_optional_strings_with_attribute() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "a={a}, b={b}", empty_str_option_not_none)]
        struct MultipleOptionals {
            a: Option<String>,
            b: Option<String>,
        }

        let template = "a=, b=value";
        let parsed = MultipleOptionals::from_str(template).unwrap();
        assert_eq!(parsed.a, Some("".into()));
        assert_eq!(parsed.b, Some("value".into()));
    }

    #[test]
    fn roundtrip_with_empty_str_option_not_none() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "data={data}", empty_str_option_not_none)]
        struct RoundtripTest {
            data: Option<String>,
        }

        let original = RoundtripTest {
            data: Some("".into()),
        };

        let template = original.render_string();
        assert_eq!(template, "data=");

        let parsed = RoundtripTest::from_str(&template).unwrap();
        assert_eq!(parsed.data, Some("".into()));
    }

    #[test]
    fn attribute_only_affects_option_string() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "text={text}, num={num}", empty_str_option_not_none)]
        struct MixedTypes {
            text: Option<String>,
            num: Option<u32>,
        }

        let template = "text=, num=42";
        let parsed = MixedTypes::from_str(template).unwrap();
        assert_eq!(parsed.text, Some("".into()));
        assert_eq!(parsed.num, Some(42));
    }

    #[test]
    fn attribute_with_missing_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "a={a}", allow_missing_placeholders, empty_str_option_not_none)]
        struct WithMissing {
            a: Option<String>,
            b: Option<String>,
        }

        let template = "a=";
        let parsed = WithMissing::from_str(template).unwrap();
        assert_eq!(parsed.a, Some("".into()));
        assert_eq!(parsed.b, None); // Not in template
    }
}

/// Tests for consecutive Option<T> placeholders
mod consecutive_option_placeholder_tests {
    use super::*;

    #[test]
    fn consecutive_option_char_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{first}{second}")]
        struct ConsecutiveOptionChars {
            first: Option<char>,
            second: Option<char>,
        }

        let chars = ConsecutiveOptionChars {
            first: Some('A'),
            second: Some('B'),
        };

        let template = chars.render_string();
        assert_eq!(template, "AB");

        let parsed = ConsecutiveOptionChars::from_str(&template).unwrap();
        assert_eq!(parsed, chars);
    }

    #[test]
    fn consecutive_option_bool_placeholders() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{flag1}{flag2}")]
        struct ConsecutiveOptionBools {
            flag1: Option<bool>,
            flag2: Option<bool>,
        }

        let bools = ConsecutiveOptionBools {
            flag1: Some(true),
            flag2: Some(false),
        };

        let template = bools.render_string();
        assert_eq!(template, "truefalse");

        let parsed = ConsecutiveOptionBools::from_str(&template).unwrap();
        assert_eq!(parsed, bools);
    }

    #[test]
    fn mixed_option_char_and_option_bool() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{character}{flag}")]
        struct MixedOptionTypes {
            character: Option<char>,
            flag: Option<bool>,
        }

        let mixed = MixedOptionTypes {
            character: Some('X'),
            flag: Some(true),
        };

        let template = mixed.render_string();
        assert_eq!(template, "Xtrue");

        let parsed = MixedOptionTypes::from_str(&template).unwrap();
        assert_eq!(parsed, mixed);
    }

    #[test]
    fn option_required_consecutive_allowed() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{opt_char}{req_bool}")]
        struct OptionAndRequired {
            opt_char: Option<char>,
            req_bool: bool,
        }

        let mixed = OptionAndRequired {
            opt_char: Some('T'),
            req_bool: false,
        };

        let template = mixed.render_string();
        assert_eq!(template, "Tfalse");

        let parsed = OptionAndRequired::from_str(&template).unwrap();
        assert_eq!(parsed, mixed);
    }

    #[test]
    fn multiple_consecutive_option_chars() {
        #[derive(Template, Debug, PartialEq)]
        #[templatia(template = "{a}{b}{c}")]
        struct MultipleOptionChars {
            a: Option<char>,
            b: Option<char>,
            c: Option<char>,
        }

        let multi = MultipleOptionChars {
            a: Some('X'),
            b: Some('Y'),
            c: Some('Z'),
        };

        let template = multi.render_string();
        assert_eq!(template, "XYZ");

        let parsed = MultipleOptionChars::from_str(&template).unwrap();
        assert_eq!(parsed, multi);
    }
}

/// Tests for Option<T> with default template
mod option_default_template_tests {
    use super::*;

    #[test]
    fn default_template_with_all_options() {
        #[derive(Template, Debug, PartialEq)]
        struct AllOptionalFields {
            name: Option<String>,
            age: Option<u32>,
            active: Option<bool>,
        }

        let instance = AllOptionalFields {
            name: Some("Alice".into()),
            age: Some(30),
            active: Some(true),
        };

        let template = instance.render_string();
        assert_eq!(template, "name = Alice\nage = 30\nactive = true");

        let parsed = AllOptionalFields::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn default_template_with_none_values() {
        #[derive(Template, Debug, PartialEq)]
        struct WithNoneValues {
            name: Option<String>,
            count: Option<u32>,
        }

        let instance = WithNoneValues {
            name: None,
            count: None,
        };

        let template = instance.render_string();
        assert_eq!(template, "name = \ncount = ");

        let parsed = WithNoneValues::from_str(&template).unwrap();
        assert_eq!(parsed.name, None);
        assert_eq!(parsed.count, None);
    }

    #[test]
    fn default_template_mixed_some_and_none() {
        #[derive(Template, Debug, PartialEq)]
        struct MixedSomeNone {
            a: Option<String>,
            b: Option<u32>,
            c: Option<bool>,
        }

        let instance = MixedSomeNone {
            a: Some("value".into()),
            b: None,
            c: Some(false),
        };

        let template = instance.render_string();
        assert_eq!(template, "a = value\nb = \nc = false");

        let parsed = MixedSomeNone::from_str(&template).unwrap();
        assert_eq!(parsed.a, Some("value".into()));
        assert_eq!(parsed.b, None);
        assert_eq!(parsed.c, Some(false));
    }

    #[test]
    fn default_template_mixed_optional_and_required() {
        #[derive(Template, Debug, PartialEq)]
        struct MixedOptionalRequired {
            required: String,
            optional: Option<String>,
        }

        let instance = MixedOptionalRequired {
            required: "must_have".into(),
            optional: Some("maybe".into()),
        };

        let template = instance.render_string();
        assert_eq!(template, "required = must_have\noptional = maybe");

        let parsed = MixedOptionalRequired::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }

    #[test]
    fn default_template_all_optional_none() {
        #[derive(Template, Debug, PartialEq)]
        struct AllNone {
            a: Option<u32>,
            b: Option<bool>,
            c: Option<String>,
        }

        let instance = AllNone {
            a: None,
            b: None,
            c: None,
        };

        let template = instance.render_string();
        assert_eq!(template, "a = \nb = \nc = ");

        let parsed = AllNone::from_str(&template).unwrap();
        assert_eq!(parsed, instance);
    }
}
