# Changelog

All notable changes to this project will be documented in this file.

## [0.0.4-alpha.1] - 2025-11-02
### Added
- Limited collection support (alpha): `Vec<T>`, `HashSet<T>`, and `BTreeSet<T>` are now supported in templates.
  - Representation: a single placeholder maps to a comma-separated list (e.g., `items={items}` with `items=a,b,c`).
  - Empty segment parses as an empty collection.
  - Duplicate placeholders must contain identical segment text.
  - `HashSet<T>` and `BTreeSet<T>` parse from the list; `HashSet<T>` naturally de-duplicates.
- Tests covering parsing, empty segments, duplicate placeholders, and error reporting for collections.

### Changed
- Updated README.md, README-ja.md, and crate-level rustdoc to document alpha collection support and installation guidance.

### Notes
- This is a pre-release for the upcoming `0.0.4`. The latest stable release remains `0.0.3`.

## [0.0.3] - 2025-10-11
### Added
- New compile-fail tests for unsupported types (collections, maps, results, tuples) to improve diagnostics and prevent incorrect usage patterns.
- New test suite `escaped_colon_tests.rs` to validate handling of escaped colons in templates.

### Changed
- Major internal refactoring: reorganized derive macro implementation by extracting `generator.rs` into modular components under `inv/` module (parser, generator, validator).
- Improved error handling with better error messages and more granular error reporting in `templatia-derive`.
- Enhanced parser logic to support future feature implementations.
- Refactored field type analysis by introducing `FieldKind` enum and `Fields` struct in `fields.rs` for better type handling and code maintainability.
- Introduced dedicated `render.rs` module for template rendering logic.

### Removed
- Decided against adding compile-time warnings for specific patterns due to complexity of implementing elegant warnings without nightly features. Focus remains on clear error messages instead.

## [0.0.2] - 2025-10-03
### Added
- Support for Option<T>: When a placeholder is absent in the template, fields of type `Option<T>` automatically default to `None`.
- New attribute `#[templatia(empty_str_option_not_none)]` to treat empty strings for `Option<String>` as `Some("")` instead of the default `None`.
- New attribute `#[templatia(allow_missing_placeholders)]` to allow fields not present in the template; such fields are initialized with `Default::default()` (requires `Default` for those field types).
- Compile-time validation to prevent ambiguous consecutive placeholders (e.g., consecutive `String`-like fields) and clearer diagnostics.
- Extensive tests: comprehensive round-trip tests, option handling tests, manual implementation examples, and compile-fail cases.
- New documentation: LIMITATION.md (current constraints) and ROADMAP_CHANGES.md (short-term roadmap). 
- Documented MSRV: Rust 1.85.0 (Edition 2024).

### Changed
- Documentation updates in README.md and README-ja.md to reflect new attributes, defaults, and examples, including explicit description of default template generation when no template is provided.
- Internal derive generator refactoring and utility/validator modules to support new parsing and validation logic.
- Updated AGENTS.md to clarify testing and documentation conventions.

### Fixed
- Improved and more consistent placeholder handling around edge cases during parsing.
- Adjusted compile-fail tests for missing fields to reflect new validation behavior.

### Breaking Changes
- Template trait API change: removed the associated type `type Struct`; `from_str` now returns `Self` instead of `Self::Struct`.
  - All manual implementations must update their signatures accordingly.
  - The derive macro has been updated to generate the new signature.
- Template method name change: `from_string` -> `from_str` and `to_string` -> `render_string`.
  - The derive macro has been updated to generate the new method names.

## [0.0.1] - 2025-09-29
- Initial release
- Publish templatia and templatia-derive