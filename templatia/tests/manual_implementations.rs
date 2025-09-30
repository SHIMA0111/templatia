use templatia::{Template, TemplateError};
// Tests follow AGENTS.md policy. They express intended behavior from docs.

/// Tests for manual Template implementations as shown in documentation
mod manual_implementation_tests {
    use super::*;

    /// Point tuple struct manual implementation from docs
    struct Point(i32, i32);

    impl Template for Point {
        type Error = TemplateError;
        type Struct = Point;

        fn to_string(&self) -> String {
            format!("({}, {})", self.0, self.1)
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            if !s.starts_with('(') || !s.ends_with(')') {
                return Err(TemplateError::Parse("Expected format: (x, y)".to_string()));
            }
            
            let inner = &s[1..s.len()-1];
            let parts: Vec<&str> = inner.split(", ").collect();
            
            if parts.len() != 2 {
                return Err(TemplateError::Parse("Expected two comma-separated values".to_string()));
            }
            
            let x = parts[0].parse().map_err(|_|
                TemplateError::Parse("Failed to parse x coordinate".to_string()))?;
            let y = parts[1].parse().map_err(|_|
                TemplateError::Parse("Failed to parse y coordinate".to_string()))?;
            
            Ok(Point(x, y))
        }
    }

    #[test]
    fn point_tuple_struct_implementation() {
        let point = Point(10, 20);
        assert_eq!(point.to_string(), "(10, 20)");

        let parsed = Point::from_string("(5, 15)").unwrap();
        assert_eq!(parsed.0, 5);
        assert_eq!(parsed.1, 15);
    }

    #[test]
    fn point_tuple_struct_roundtrip() {
        let original = Point(-100, 200);
        let template = original.to_string();
        let parsed = Point::from_string(&template).unwrap();
        assert_eq!(parsed.0, original.0);
        assert_eq!(parsed.1, original.1);
    }

    #[test]
    fn point_tuple_struct_parse_errors() {
        // Missing parentheses
        let result = Point::from_string("10, 20");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Wrong number of values
        let result = Point::from_string("(10, 20, 30)");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Invalid numbers
        let result = Point::from_string("(abc, 20)");
        assert!(matches!(result, Err(TemplateError::Parse(_))));
    }

    /// ServerConfig manual implementation from docs
    struct ServerConfig {
        name: String,
        port: u16,
    }

    impl Template for ServerConfig {
        type Error = TemplateError;
        type Struct = ServerConfig;

        fn to_string(&self) -> String {
            format!("server={},port={}", self.name, self.port)
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 2 {
                return Err(TemplateError::Parse("Expected format: server=name,port=number".to_string()));
            }

            let name = parts[0].strip_prefix("server=")
                .ok_or_else(|| TemplateError::Parse("Missing server= prefix".to_string()))?
                .to_string();
                
            let port_str = parts[1].strip_prefix("port=")
                .ok_or_else(|| TemplateError::Parse("Missing port= prefix".to_string()))?;
                
            let port = port_str.parse::<u16>()
                .map_err(|_| TemplateError::Parse("Invalid port number".to_string()))?;

            Ok(ServerConfig { name, port })
        }
    }

    #[test]
    fn server_config_manual_implementation() {
        let config = ServerConfig { name: "web01".to_string(), port: 8080 };
        let template = config.to_string();
        assert_eq!(template, "server=web01,port=8080");

        let parsed = ServerConfig::from_string(&template).unwrap();
        assert_eq!(parsed.name, "web01");
        assert_eq!(parsed.port, 8080);
    }

    #[test]
    fn server_config_parse_errors() {
        // Wrong format
        let result = ServerConfig::from_string("web01:8080");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Missing server= prefix
        let result = ServerConfig::from_string("web01,port=8080");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Missing port= prefix
        let result = ServerConfig::from_string("server=web01,8080");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Invalid port
        let result = ServerConfig::from_string("server=web01,port=abc");
        assert!(matches!(result, Err(TemplateError::Parse(_))));
    }

    /// Generic KeyValue implementation from docs
    struct KeyValue<T> {
        key: String,
        value: T,
    }

    impl<T> Template for KeyValue<T>
    where
        T: std::fmt::Display + std::str::FromStr + Clone,
        T::Err: std::fmt::Display,
    {
        type Error = TemplateError;
        type Struct = KeyValue<T>;

        fn to_string(&self) -> String {
            format!("{}={}", self.key, self.value)
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            let parts: Vec<&str> = s.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(TemplateError::Parse("Expected key=value format".to_string()));
            }

            let key = parts[0].to_string();
            let value = parts[1].parse::<T>()
                .map_err(|e| TemplateError::Parse(format!("Failed to parse value: {}", e)))?;

            Ok(KeyValue { key, value })
        }
    }

    #[test]
    fn generic_key_value_string() {
        let config = KeyValue { key: "name".to_string(), value: "test".to_string() };
        assert_eq!(config.to_string(), "name=test");

        let parsed = KeyValue::<String>::from_string("title=hello world").unwrap();
        assert_eq!(parsed.key, "title");
        assert_eq!(parsed.value, "hello world");
    }

    #[test]
    fn generic_key_value_numeric() {
        let config = KeyValue { key: "timeout".to_string(), value: 30u32 };
        assert_eq!(config.to_string(), "timeout=30");

        let parsed = KeyValue::<u32>::from_string("retry=5").unwrap();
        assert_eq!(parsed.key, "retry");
        assert_eq!(parsed.value, 5);
    }

    #[test]
    fn generic_key_value_boolean() {
        let config = KeyValue { key: "enabled".to_string(), value: true };
        assert_eq!(config.to_string(), "enabled=true");

        let parsed = KeyValue::<bool>::from_string("debug=false").unwrap();
        assert_eq!(parsed.key, "debug");
        assert_eq!(parsed.value, false);
    }

    #[test]
    fn generic_key_value_parse_errors() {
        // No equals sign
        let result = KeyValue::<u32>::from_string("timeout30");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Invalid numeric value
        let result = KeyValue::<u32>::from_string("timeout=abc");
        assert!(matches!(result, Err(TemplateError::Parse(_))));
    }

    /// Custom error type implementation
    #[derive(Debug)]
    struct CustomError(String);

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Custom error: {}", self.0)
        }
    }

    impl std::error::Error for CustomError {}

    #[derive(Debug)]
    struct CustomConfig {
        value: String,
    }

    impl Template for CustomConfig {
        type Error = CustomError;
        type Struct = CustomConfig;

        fn to_string(&self) -> String {
            format!("custom:{}", self.value)
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            if let Some(value) = s.strip_prefix("custom:") {
                Ok(CustomConfig { value: value.to_string() })
            } else {
                Err(CustomError("Missing custom: prefix".to_string()))
            }
        }
    }

    #[test]
    fn custom_error_type() {
        let config = CustomConfig { value: "test".to_string() };
        assert_eq!(config.to_string(), "custom:test");

        let parsed = CustomConfig::from_string("custom:hello").unwrap();
        assert_eq!(parsed.value, "hello");

        let result = CustomConfig::from_string("invalid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Custom error: Missing custom: prefix");
    }
}

/// Tests for edge cases in manual implementations
mod manual_implementation_edge_cases {
    use super::*;

    /// Unit struct implementation
    struct Unit;

    impl Template for Unit {
        type Error = TemplateError;
        type Struct = Unit;

        fn to_string(&self) -> String {
            "unit".to_string()
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            if s == "unit" {
                Ok(Unit)
            } else {
                Err(TemplateError::Parse("Expected 'unit'".to_string()))
            }
        }
    }

    #[test]
    fn unit_struct_implementation() {
        let unit = Unit;
        assert_eq!(unit.to_string(), "unit");

        let parsed = Unit::from_string("unit").unwrap();
        let _ = parsed; // Unit has no fields to check

        let result = Unit::from_string("invalid");
        assert!(matches!(result, Err(TemplateError::Parse(_))));
    }

    /// Empty template implementation
    struct Empty {
        _phantom: std::marker::PhantomData<()>,
    }

    impl Template for Empty {
        type Error = TemplateError;
        type Struct = Empty;

        fn to_string(&self) -> String {
            "".to_string()
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            if s.is_empty() {
                Ok(Empty { _phantom: std::marker::PhantomData })
            } else {
                Err(TemplateError::Parse("Expected empty string".to_string()))
            }
        }
    }

    #[test]
    fn empty_template_implementation() {
        let empty = Empty { _phantom: std::marker::PhantomData };
        assert_eq!(empty.to_string(), "");

        let parsed = Empty::from_string("").unwrap();
        let _ = parsed; // No fields to check

        let result = Empty::from_string("not empty");
        assert!(matches!(result, Err(TemplateError::Parse(_))));
    }

    /// Complex nested structure
    struct ComplexNested {
        outer: String,
        inner: Vec<String>,
    }

    impl Template for ComplexNested {
        type Error = TemplateError;
        type Struct = ComplexNested;

        fn to_string(&self) -> String {
            format!("{}:[{}]", self.outer, self.inner.join(","))
        }

        fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
            if let Some((outer_part, inner_part)) = s.split_once(':') {
                if let Some(inner_content) = inner_part.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                    let inner = if inner_content.is_empty() {
                        Vec::new()
                    } else {
                        inner_content.split(',').map(|s| s.to_string()).collect()
                    };
                    
                    Ok(ComplexNested {
                        outer: outer_part.to_string(),
                        inner,
                    })
                } else {
                    Err(TemplateError::Parse("Invalid inner format, expected [...]".to_string()))
                }
            } else {
                Err(TemplateError::Parse("Expected outer:inner format".to_string()))
            }
        }
    }

    #[test]
    fn complex_nested_implementation() {
        let complex = ComplexNested {
            outer: "test".to_string(),
            inner: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };
        
        assert_eq!(complex.to_string(), "test:[a,b,c]");

        let parsed = ComplexNested::from_string("hello:[x,y,z]").unwrap();
        assert_eq!(parsed.outer, "hello");
        assert_eq!(parsed.inner, vec!["x", "y", "z"]);

        // Empty inner
        let empty_inner = ComplexNested::from_string("empty:[]").unwrap();
        assert_eq!(empty_inner.outer, "empty");
        assert!(empty_inner.inner.is_empty());
    }

    #[test]
    fn complex_nested_parse_errors() {
        // Missing colon
        let result = ComplexNested::from_string("no_colon");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Missing brackets
        let result = ComplexNested::from_string("test:no_brackets");
        assert!(matches!(result, Err(TemplateError::Parse(_))));

        // Mismatched brackets
        let result = ComplexNested::from_string("test:[missing_close");
        assert!(matches!(result, Err(TemplateError::Parse(_))));
    }
}