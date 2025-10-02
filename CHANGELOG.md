# Changelog

All notable changes to this project will be documented in this file.

## [0.0.2] - 2025-10-02
### Added
- Support for Option<T>: When a placeholder is absent in the template, fields of type `Option<T>` automatically default to `None`.
- New attribute `#[templatia(empty_str_option_not_none)]` to treat empty strings for `Option<String>` as `Some("")` instead of the default `None`.
- New attribute `#[templatia(allow_missing_placeholders)]` to allow fields not present in the template; such fields are initialized with `Default::default()` (requires `Default` for those field types).
- Compile-time validation to prevent ambiguous consecutive placeholders (e.g., consecutive `String`-like fields) and clearer diagnostics.
- Extensive tests: comprehensive round-trip tests, option handling tests, manual implementation examples, and compile-fail cases.

### Changed
- Documentation updates in README.md and README-ja.md to reflect new attributes, defaults, and examples.
- Internal derive generator refactoring and utility/validator modules to support new parsing and validation logic.

### Fixed
- Improved and more consistent placeholder handling around edge cases during parsing.
- Adjusted compile-fail tests for missing fields to reflect new validation behavior.

### Breaking Changes
- Template trait API change: removed the associated type `type Struct`; `from_string` now returns `Self` instead of `Self::Struct`.
  - All manual implementations must update their signatures accordingly.
  - The derive macro has been updated to generate the new signature.
- Template method name change: `from_string` -> `from_str` and `to_string` -> `render_string`.
  - The derive macro has been updated to generate the new method names.

## [0.0.1] - 2025-09-29
- Initial release
- Publish templatia and templatia-derive