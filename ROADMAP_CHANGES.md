# Roadmap Changes

This document tracks significant changes to the project roadmap and explains the reasoning behind those changes.

## v0.0.3

### Original Plan

The initial v0.0.3 roadmap included:

1. Enrich error handling and warnings (clearer diagnostics and coverage)
2. Add compile-time warnings for specific usage patterns

### Changes Made

**Removed Items:**
- **Compile-time warnings**: Removed because implementing elegant warnings without nightly compiler features (such as `proc_macro::Diagnostic`) would require workarounds that compromise code quality and maintainability. Instead, the focus shifted to providing clear and actionable compile-time errors for unsupported patterns.

**What Was Actually Done:**
- Enhanced error messages and added comprehensive compile-fail tests for unsupported types (collections, maps, results, tuples)
- Major internal refactoring to prepare for future feature implementations
- Improved parser logic and error handling infrastructure
- Added test coverage for edge cases (escaped colons, etc.)

### Design Rationale

The revised v0.0.3 approach follows these principles:

1. **Pragmatism over perfection**: Clear error messages are more valuable than warnings when the action is definitively wrong
2. **Maintainability**: Avoid complex workarounds that would make the codebase harder to maintain
3. **Future-proofing**: Internal refactoring sets up a solid foundation for upcoming features like collections support in v0.0.4

## v0.0.2

### Original Plan

The initial v0.0.2 roadmap included four items:

1. Emit warnings when not all struct fields are present in the template during parsing
2. Define default behavior for missing data: decide behavior when some or all fields are not included in the template
3. Option<T>: default to `None` when the placeholder is absent
4. String: add configurable handler for missing placeholders (error vs `String::new()`)

### Changes Made

**Removed Items:**
- **Field missing warnings**: Removed because it creates redundancy. When users explicitly specify `#[templatia(allow_missing_placeholders)]`, they are declaring their intent to allow missing fields. Adding warnings in this case would be contradictory and verbose.

- **String-specific configurable handler**: Removed due to lack of consistency. There is no justification for treating `String` fields differently from other types like `u32`, `bool`, or `char`. All types use `Default::default()` when missing, which provides a uniform and predictable behavior. Additionally, the existing `allow_missing_placeholders` attribute already provides the binary choice (error vs default), making a String-specific option redundant.

**Added Items:**
- **Remove `type::Struct` from Template trait**: Added to reduce complexity. The trait is implemented for a struct, the type of the struct should be itself.

### Design Rationale

The revised v0.0.2 roadmap follows these principles:

1. **Simplicity over complexity**: Binary choices (allow missing vs error) are clearer than multi-option configurations. Also, the type::Struct removal reduces complexity.
2. **Consistency**: Treat all types uniformly; avoid special cases for specific types like `String`
3. **Declarative intent**: Use attributes to make behavior explicit and self-documenting
