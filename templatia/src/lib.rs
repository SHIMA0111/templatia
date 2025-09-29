//! # Templatia
//!
//! A powerful and easy-to-use library for converting Rust structs to and from text templates.
//! Templatia lets you define templates with placeholders and automatically handles the
//! serialization and deserialization, making configuration management and text processing
//! effortless.
//!
//! ## Key Features
//!
//! - **🚀 Simple**: Just add `#[derive(Template)]` to your struct
//! - **🔧 Flexible**: Use default templates or define custom ones
//! - **🛡️ Safe**: Type-safe parsing with clear error reporting
//! - **🔄 Round-trip**: Reliable conversion to/from strings
//!
//! ## Quick Start
//!
//! Add templatia to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! templatia = { version = "0.0.1", features = ["derive"] }
//! ```
//!
//! ### Using the Derive Macro (Recommended)
//!
//! The easiest way to use templatia is with the derive macro for named structs:
//!
//! ```rust
//! use templatia::Template;
//!
//! #[derive(Template)]
//! struct DatabaseConfig {
//!     host: String,
//!     port: u16,
//!     database: String,
//! }
//!
//! let config = DatabaseConfig {
//!     host: "localhost".to_string(),
//!     port: 5432,
//!     database: "myapp".to_string(),
//! };
//!
//! // Convert to template string (default format: field = {field})
//! let template = config.to_string();
//! assert_eq!(template, "host = localhost\nport = 5432\ndatabase = myapp");
//!
//! // Parse back from template string
//! let parsed = DatabaseConfig::from_string(&template).unwrap();
//! assert_eq!(parsed.host, "localhost");
//! assert_eq!(parsed.port, 5432);
//! ```
//!
//! ### Custom Templates
//!
//! Define your own template format using the `templatia` attribute:
//!
//! ```rust
//! use templatia::Template;
//!
//! #[derive(Template)]
//! #[templatia(template = "postgresql://{host}:{port}/{database}")]
//! struct PostgresUrl {
//!     host: String,
//!     port: u16,
//!     database: String,
//! }
//!
//! let url = PostgresUrl {
//!     host: "db.example.com".to_string(),
//!     port: 5432,
//!     database: "production".to_string(),
//! };
//!
//! assert_eq!(url.to_string(), "postgresql://db.example.com:5432/production");
//!
//! let parsed = PostgresUrl::from_string("postgresql://localhost:5432/test").unwrap();
//! assert_eq!(parsed.database, "test");
//! ```
//!
//! ### Advanced Features
//!
//! #### Duplicate Placeholders
//! Templatia supports duplicate placeholders as long as they have consistent values:
//!
//! ```rust
//! use templatia::Template;
//!
//! #[derive(Template)]
//! #[templatia(template = "Welcome {name}! Your name is {name}.")]
//! struct Greeting {
//!     name: String,
//! }
//!
//! let greeting = Greeting { name: "Alice".to_string() };
//! assert_eq!(greeting.to_string(), "Welcome Alice! Your name is Alice.");
//!
//! // Parsing with inconsistent values will result in an error
//! let result = Greeting::from_string("Welcome Alice! Your name is Bob.");
//! assert!(result.is_err());
//! ```
//!
//! ### Manual Implementation (Advanced)
//!
//! While the derive macro only supports named structs currently, you can manually implement
//! the `Template` trait for other types like tuple structs, enums, or complex custom logic:
//!
//! ```rust
//! use templatia::{Template, TemplateError};
//!
//! // Example: Tuple struct (derive doesn't support this yet)
//! struct Point(i32, i32);
//!
//! impl Template for Point {
//!     type Error = TemplateError;
//!     type Struct = Point;
//!
//!     fn to_string(&self) -> String {
//!         format!("({}, {})", self.0, self.1)
//!     }
//!
//!     fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
//!         if !s.starts_with('(') || !s.ends_with(')') {
//!             return Err(TemplateError::Parse("Expected format: (x, y)".to_string()));
//!         }
//!         
//!         let inner = &s[1..s.len()-1];
//!         let parts: Vec<&str> = inner.split(", ").collect();
//!         
//!         if parts.len() != 2 {
//!             return Err(TemplateError::Parse("Expected two comma-separated values".to_string()));
//!         }
//!         
//!         let x = parts[0].parse().map_err(|_|
//!             TemplateError::Parse("Failed to parse x coordinate".to_string()))?;
//!         let y = parts[1].parse().map_err(|_|
//!             TemplateError::Parse("Failed to parse y coordinate".to_string()))?;
//!         
//!         Ok(Point(x, y))
//!     }
//! }
//!
//! let point = Point(10, 20);
//! assert_eq!(point.to_string(), "(10, 20)");
//!
//! let parsed = Point::from_string("(5, 15)").unwrap();
//! assert_eq!(parsed.0, 5);
//! assert_eq!(parsed.1, 15);
//! ```
//!
//! ## Load Map (Roadmap)
//!
//! Templatia follows a clear development roadmap with planned features:
//!
//! ### 🚧 Version 0.0.2
//! - Enhanced error handling with warnings for missing fields
//! - Configurable default behavior for missing data
//! - `Option<T>` support (defaults to `None` when placeholder absent)
//! - String field configuration for missing placeholders
//!
//! ### 🔮 Version 0.0.3
//! - Enriched error diagnostics and coverage
//! - More detailed parsing error messages
//!
//! ### 🎯 Version 0.0.4  
//! - Collections support: `Vec`, `HashMap`, `HashSet`
//! - Container attributes for flexible parent structure configuration
//!
//! ### 🌟 Version 0.0.5
//! - Tuple struct support (derive macro)
//! - Union struct support  
//! - Enum support
//!
//! ## Type Requirements
//!
//! Fields used in templates must implement:
//! - `std::fmt::Display` for serialization
//! - `std::str::FromStr` for deserialization  
//! - `std::cmp::PartialEq` for consistency checks
//!
//! Most common types (String, integers, floats, bool) implement these automatically.
//!
//! ## Error Handling
//!
//! Templatia provides clear error types for different failure scenarios:
//!
//! ```rust
//! use templatia::{Template, TemplateError};
//!
//! #[derive(Template)]
//! #[templatia(template = "port={port}")]
//! struct Config { port: u16 }
//!
//! // Parse error example
//! match Config::from_string("port=not_a_number") {
//!     Err(TemplateError::Parse(msg)) => println!("Parse failed: {}", msg),
//!     _ => unreachable!(),
//! }
//!
//! // Inconsistent values error (when same placeholder appears multiple times)
//! #[derive(Template)]
//! #[templatia(template = "id={id}-backup-{id}")]
//! struct BackupConfig { id: String }
//!
//! match BackupConfig::from_string("id=prod-backup-dev") {
//!     Err(TemplateError::InconsistentValues { placeholder, first_value, second_value }) => {
//!         println!("Placeholder '{}' had conflicting values: '{}' vs '{}'",
//!                  placeholder, first_value, second_value);
//!     },
//!     _ => unreachable!(),
//! }
//! ```

#[cfg(feature = "derive")]
#[doc(inline)]
pub use templatia_derive::Template;

/// A trait for converting between a struct and its string template form.
///
/// This trait enables bidirectional conversion between Rust data structures and their
/// string template representations. It's the core abstraction that powers templatia's
/// serialization and deserialization capabilities.
///
/// # Design Philosophy
///
/// The `Template` trait is designed to be:
/// - **Predictable**: Round-trip conversion should preserve data integrity
/// - **Flexible**: Support various template formats and data structures  
/// - **Type-safe**: Leverage Rust's type system for compile-time guarantees
/// - **Error-aware**: Provide clear feedback when parsing fails
///
/// # Associated Types
///
/// - `Error`: The concrete error type returned by parsing operations. Should implement
///   `std::error::Error + std::fmt::Display` for best integration with error handling.
/// - `Struct`: The concrete struct type produced by `from_string`. This allows the trait
///   to work with generic implementations while maintaining type safety.
///
/// # Implementation Guidelines
///
/// When manually implementing this trait:
///
/// 1. **Consistency**: Ensure `from_string(x.to_string())` equals `x` for valid data
/// 2. **Error Handling**: Use descriptive error messages that help users debug issues
/// 3. **Performance**: Consider caching compiled parsers for repeated use
/// 4. **Validation**: Validate data integrity, especially with duplicate placeholders
///
/// # Examples
///
/// ## Basic Implementation
///
/// ```rust
/// use templatia::{Template, TemplateError};
///
/// struct ServerConfig {
///     name: String,
///     port: u16,
/// }
///
/// impl Template for ServerConfig {
///     type Error = TemplateError;
///     type Struct = ServerConfig;
///
///     fn to_string(&self) -> String {
///         format!("server={},port={}", self.name, self.port)
///     }
///
///     fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
///         let parts: Vec<&str> = s.split(',').collect();
///         if parts.len() != 2 {
///             return Err(TemplateError::Parse("Expected format: server=name,port=number".to_string()));
///         }
///
///         let name = parts[0].strip_prefix("server=")
///             .ok_or_else(|| TemplateError::Parse("Missing server= prefix".to_string()))?
///             .to_string();
///             
///         let port_str = parts[1].strip_prefix("port=")
///             .ok_or_else(|| TemplateError::Parse("Missing port= prefix".to_string()))?;
///             
///         let port = port_str.parse::<u16>()
///             .map_err(|_| TemplateError::Parse("Invalid port number".to_string()))?;
///
///         Ok(ServerConfig { name, port })
///     }
/// }
///
/// let config = ServerConfig { name: "web01".to_string(), port: 8080 };
/// let template = config.to_string();
/// assert_eq!(template, "server=web01,port=8080");
///
/// let parsed = ServerConfig::from_string(&template).unwrap();
/// assert_eq!(parsed.name, "web01");
/// assert_eq!(parsed.port, 8080);
/// ```
///
/// ## Generic Implementation
///
/// ```rust
/// use templatia::{Template, TemplateError};
/// use std::fmt::Display;
/// use std::str::FromStr;
///
/// struct KeyValue<T> {
///     key: String,
///     value: T,
/// }
///
/// impl<T> Template for KeyValue<T>
/// where
///     T: Display + FromStr + Clone,
///     T::Err: Display,
/// {
///     type Error = TemplateError;
///     type Struct = KeyValue<T>;
///
///     fn to_string(&self) -> String {
///         format!("{}={}", self.key, self.value)
///     }
///
///     fn from_string(s: &str) -> Result<Self::Struct, Self::Error> {
///         let parts: Vec<&str> = s.splitn(2, '=').collect();
///         if parts.len() != 2 {
///             return Err(TemplateError::Parse("Expected key=value format".to_string()));
///         }
///
///         let key = parts[0].to_string();
///         let value = parts[1].parse::<T>()
///             .map_err(|e| TemplateError::Parse(format!("Failed to parse value: {}", e)))?;
///
///         Ok(KeyValue { key, value })
///     }
/// }
///
/// let config = KeyValue { key: "timeout".to_string(), value: 30u32 };
/// assert_eq!(config.to_string(), "timeout=30");
///
/// let parsed = KeyValue::<u32>::from_string("retry=5").unwrap();
/// assert_eq!(parsed.value, 5);
/// ```
pub trait Template {
    /// The concrete error type for template parsing or formatting failures.
    ///
    /// This should typically be `TemplateError` for most implementations, but custom
    /// error types are supported for specialized use cases. The error type should
    /// implement `std::error::Error` for best integration with Rust's error ecosystem.
    type Error;

    /// The concrete struct type created by `from_string`.
    ///
    /// This is usually the same as `Self`, but using an associated type allows for
    /// more flexible implementations, particularly with generic types or when the
    /// parsing result might differ from the input type.
    type Struct;

    /// Converts the value into its template string representation.
    ///
    /// This method serializes the struct into a string format according to the
    /// defined template rules. The output should be parseable by `from_string`
    /// to maintain round-trip consistency.
    ///
    /// # Performance Notes
    ///
    /// This method creates a new `String` on each call. For performance-critical
    /// applications with frequent serialization, consider caching results or using
    /// streaming approaches.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use templatia::Template;
    ///
    /// #[derive(Template)]
    /// struct AppConfig {
    ///     name: String,
    ///     debug: bool,
    /// }
    ///
    /// let config = AppConfig {
    ///     name: "myapp".to_string(),
    ///     debug: true,
    /// };
    ///
    /// let template = config.to_string();
    /// // Default format: "name = myapp\ndebug = true"
    /// assert!(template.contains("name = myapp"));
    /// assert!(template.contains("debug = true"));
    /// ```
    fn to_string(&self) -> String;

    /// Parses an instance from a template string.
    ///
    /// This method deserializes a string into the target struct type according to
    /// the defined template rules. It should be the inverse operation of `to_string`.
    ///
    /// # Parameters
    ///
    /// * `s` - The source string to parse. Should match the expected template format.
    ///
    /// # Returns
    ///
    /// On success, returns a constructed instance of `Self::Struct`. The result
    /// should be equivalent to the original struct that generated the string.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` when:
    /// - The string format doesn't match the expected template
    /// - Field values cannot be parsed to their target types
    /// - Duplicate placeholders have inconsistent values
    /// - Required placeholders are missing from the template
    ///
    /// # Examples
    ///
    /// ```rust
    /// use templatia::{Template, TemplateError};
    ///
    /// #[derive(Template, PartialEq, Debug)]
    /// #[templatia(template = "host={host}:{port}")]
    /// struct Connection {
    ///     host: String,
    ///     port: u16,
    /// }
    ///
    /// // Successful parsing
    /// let conn = Connection::from_string("host=localhost:8080").unwrap();
    /// assert_eq!(conn.host, "localhost");
    /// assert_eq!(conn.port, 8080);
    ///
    /// // Error handling
    /// match Connection::from_string("host=localhost:invalid_port") {
    ///     Err(TemplateError::Parse(msg)) => {
    ///         println!("Parsing failed: {}", msg);
    ///     }
    ///     _ => panic!("Expected parse error"),
    /// }
    /// ```
    fn from_string(s: &str) -> Result<Self::Struct, Self::Error>;
}

/// Errors produced by templatia operations.
///
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    /// The same placeholder occurred multiple times with different values.
    ///
    /// # Parameters
    /// - placeholder: The placeholder name.
    /// - first_value: The first observed value.
    /// - second_value: The conflicting later value.
    #[error(
        "Inconsistent values for placeholder '{placeholder}': found '{first_value}', and after face '{second_value}'"
    )]
    InconsistentValues {
        placeholder: String,
        first_value: String,
        second_value: String,
    },
    /// A generic parse error message aggregated from the parser.
    #[error("Parse error: {0}")]
    Parse(String),
}

#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod __private {
    pub use chumsky;
}
