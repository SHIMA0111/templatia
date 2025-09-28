//! Minimal template trait and error types for templatia.
//!
//! This crate defines the core Template trait and its error type used by the
//! templatia-derive procedural macro.
//!
//! # Examples
//! ```rust
//! use templatia::Template;
//!
//! // A consumer typically uses the derive macro from `templatia-derive`.
//! // The example below demonstrates the trait surface only.
//! struct Mock;
//!
//! impl Template for Mock {
//!     type Error = templatia::TemplateError;
//!     type Struct = Mock;
//!
//!     fn to_string(&self) -> String { String::from("mock") }
//!     fn from_string(_s: &str) -> Result<Self::Struct, Self::Error> { Ok(Mock) }
//! }
//! assert_eq!(Mock.to_string(), "mock");
//! ```

#[cfg(feature = "derive")]
pub mod derive {
    pub use templatia_derive::*;
}

/// A trait for converting between a struct and its string template form.
///
/// Types implementing this trait can serialize themselves into a template
/// string and be reconstructed from a string.
///
/// # Associated Types
/// - Error: Concrete error type used by the implementation.
/// - Struct: The concrete struct type to be produced by `from_string`.
pub trait Template {
    /// Concrete error type for template parsing or formatting failures.
    type Error;
    /// The struct type created by `from_string`.
    type Struct;

    /// Converts the value into its template string representation.
    ///
    /// # Returns
    /// A `String` that represents `self` according to the template rules.
    ///
    /// # Examples
    /// ```rust
    /// use templatia::Template;
    ///
    /// struct S;
    /// impl Template for S {
    ///     type Error = templatia::TemplateError;
    ///     type Struct = S;
    ///     fn to_string(&self) -> String { "x".into() }
    ///     fn from_string(_: &str) -> Result<Self::Struct, Self::Error> { Ok(S) }
    /// }
    /// assert_eq!(S.to_string(), "x");
    /// ```
    fn to_string(&self) -> String;

    /// Parses an instance from a string.
    ///
    /// # Parameters
    /// - s: Source string to parse.
    ///
    /// # Returns
    /// On success, returns a constructed instance of `Self::Struct`.
    ///
    /// # Errors
    /// Returns `Self::Error` if the string cannot be parsed.
    ///
    /// # Examples
    /// ```rust
    /// use templatia::Template;
    ///
    /// struct S;
    /// impl Template for S {
    ///     type Error = templatia::TemplateError;
    ///     type Struct = S;
    ///     fn to_string(&self) -> String { String::new() }
    ///     fn from_string(_: &str) -> Result<Self::Struct, Self::Error> { Ok(S) }
    /// }
    /// let _ = S::from_string("").unwrap();
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
    #[error("Inconsistent values for placeholder '{placeholder}': found '{first_value}', and after face '{second_value}'")]
    InconsistentValues {
        placeholder: String,
        first_value: String,
        second_value: String,
    },
    /// A generic parse error message aggregated from the parser.
    #[error("Parse error: {0}")]
    Parse(String),
}