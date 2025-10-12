[![CI](https://github.com/SHIMA0111/templatia/actions/workflows/ci.yml/badge.svg)](https://github.com/SHIMA0111/templatia/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/templatia.svg)](https://crates.io/crates/templatia)
[![Docs.rs](https://docs.rs/templatia/badge.svg)](https://docs.rs/templatia)
[![Crates.io MSRV (version)](https://img.shields.io/crates/msrv/templatia/0.0.3)](https://crates.io/crates/templatia)
[![Downloads](https://img.shields.io/crates/d/templatia.svg)](https://crates.io/crates/templatia)

# templatia

A template-based serialization/deserialization library that enables seamless bidirectional conversion between Rust structs and text according to user-defined templates.
This library is realized through two crates:

- templatia: A library providing core traits and errors
- templatia-derive: A macro library that automatically generates logic for bidirectional conversion between structs and text according to user templates

Typically, these are not used individually but combined via the `derive` feature of `templatia`.
(However, since templatia-derive currently only supports `named_struct`, custom implementations are also possible for special types.)

## Features
- Seamless bidirectional conversion between Rust structs and text
- Default template with all fields in key-value format: `{field_name} = {field_name}`
- Custom template definition using `templatia` attribute: `#[templatia(template = "...")]`
- Clear runtime errors and understandable compile errors
  - Examples of compile errors
    - Compile error for consecutive ambiguous combinations
      - StructName: Placeholder "field1" and "field2" are consecutive. These are ambiguous to parsing.
        "field1" is `String` type data. Consecutive allows only: [char, bool]
    - Compile error when not all struct fields are included in template placeholders
      - StructName has more field specified than the template's placeholders: field1, field2, field3
        If you want to allow missing placeholders, use `#[templatia(allow_missing_placeholders)]` attribute.

## Minimum supported Rust version (MSRV)
- Rust 1.85.0
- Edition 2024

## Installation
### Using the cargo add command
```shell
cargo add templatia --features derive
```

### Direct specification in Cargo.toml
1) Import templatia. Add `derive` to features.

```toml
[dependencies]
templatia = { version = "0.0.3", features = ["derive"] }
```

```rust
use templatia::Template; 

#[derive(Template)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    let cfg = Config { host: "localhost".into(), port: 5432 };
    let s = cfg.render_string();
    assert!(s.contains("host = localhost"));
    assert!(s.contains("port = 5432"));
}
```


## Quick Start Guide
### Default template
When no template is specified, each field is synthesized in the format `field_name = {field_name}`, one per line.
For example:
```rust
#[derive(Template)]
struct AwesomeStruct {
  data1: String,
  data2: u32,
}

fn main() {
  let data = AwesomeStruct { data1: "data1".into(), data2: 100 };
}
```
In this case, the template is generated in the format:
```text
data1 = {data1}
data2 = {data2}
```
When executing render_string(), you get the output:
```text
data1 = data1
data2 = 100
```

### Custom template
By using placeholders enclosed in `{}` with struct field names in the `template` within the `templatia` attribute, you can define a custom template.
In the following case, since `"{host}:{port}"` is defined, you can obtain `db.example.com:3306` from `cfg`.

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "{host}:{port}")]
struct DbCfg {
    host: String,
    port: u16,
}

fn main() {
    let cfg = DbCfg { host: "db.example.com".into(), port: 3306 };
    assert_eq!(cfg.render_string(), "db.example.com:3306");

    let parsed = DbCfg::from_str("db.example.com:3306").unwrap();
    assert_eq!(parsed.host, "db.example.com");
    assert_eq!(parsed.port, 3306);
}
```

### Option<T> support
Fields with `Option<T>` type automatically default to `None` when the placeholder is not present in the template:

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "host={host}:{port}", allow_missing_placeholders)]
struct ServerConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

fn main() {
    let config = ServerConfig::from_str("host=localhost:8080").unwrap();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert_eq!(config.username, None); // Not in template, defaults to None
    assert_eq!(config.password, None); // Not in template, defaults to None
}
```

By default, empty strings in `Option<String>` are parsed as `None`. To treat empty strings as `Some("")`, use the `empty_str_option_not_none` attribute:

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "value={value}", empty_str_option_not_none)]
struct OptionalValue {
    value: Option<String>,
}

fn main() {
    let parsed = OptionalValue::from_str("value=").unwrap();
    assert_eq!(parsed.value, Some("".to_string())); // Empty string becomes Some("")
}
```

### Missing placeholders
Use the `allow_missing_placeholders` attribute to allow fields that are not present in the template:

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "id={id}", allow_missing_placeholders)]
struct Config {
    id: u32,
    name: String,           // Not in template, uses Default::default()
    optional: Option<u32>,  // Not in template, becomes None
}

fn main() {
    let config = Config::from_str("id=42").unwrap();
    assert_eq!(config.id, 42);
    assert_eq!(config.name, "");          // Default for String
    assert_eq!(config.optional, None);     // None for Option<T>
}
```

### Placeholders and types
- Each `{name}` in the template must correspond to a named struct field
- Field types used in the template must implement Display and FromStr
  - When `allow_missing_placeholders` is enabled, the Default trait implementation is also required.
- It is possible to use placeholders for the same field multiple times within the template, but during from_str() the placeholders for the same field must have the same value.
  - For example, if the template is `"{first_name} (Full: {first_name} {family_name})"`, you cannot deserialize `Taro (Full: Jiro Yamada)` into the struct.

## Runtime Errors
templatia defines a simple error type for parsing and validation:

- TemplateError::InconsistentValues { placeholder, first_value, second_value }
  - Emitted when the same placeholder appears multiple times with conflicting parsed values
- TemplateError::ParseToType { placeholder, value, type_name }
  - Parse error when the value cannot be parsed to the specified type
- TemplateError::UnexpectedInput { expected_next_literal, remaining_text }
  - Input string literal does not match the specified template
- TemplateError::Parse(String)
  - Generic parse error message

## Crates overview
- templatia
  - Template trait
    - A trait that defines the behavior of `templatia`.
      It defines two methods: `render_string()` and `from_str()`, and one associated type: `Error`.
  - TemplateError enum for error reporting
- templatia-derive
  - #[derive(Template)] macro for named structs
  - Optional attributes:
    - `#[templatia(template = "...")]` for custom templates
    - `#[templatia(allow_missing_placeholders)]` to allow fields not in template
    - `#[templatia(empty_str_option_not_none)]` to treat empty strings as `Some("")` for `Option<String>`
  - Validates that placeholders exist as fields

## Feature flags
- derive
  - A flag that enables templatia-derive. By enabling this, you can derive `templatia::Template`.

## Road Map (0.0.x roadmap toward 0.1.0)
- 0.0.2
  - [x] Define default behavior for missing data: `#[templatia(allow_missing_placeholders)]` attribute allows fields not in template to use `Default::default()`
  - [x] Option<T>: default to `None` when the placeholder is absent (automatic support without requiring `allow_missing_placeholders`)
  - [x] Remove `type Struct` from `Template` trait
- 0.0.3
  - [x] Enrich error handling (clearer diagnostics and coverage through compile-fail tests)
  - [x] Internal refactoring to prepare for future feature implementations
- 0.0.4
  - [ ] Declarative templates for field collections such as `Vec`, `HashMap`, and `HashSet`
  - [ ] Add `container` attribute to increase flexibility at the parent structure level
- 0.0.5
  - [ ] Support additional data forms: tuple (unnamed) structs, union structs, and enums
- 0.0.6 and beyond (Future versions)
  - [ ] Optional placeholder syntax: `{name?}` to make individual placeholders optional
    - For `Option<T>` fields, treat the placeholder as empty string when value is `None`
    - During parsing, return `None` when the placeholder is absent
  - [ ] Range optional syntax: `[literal{placeholder}literal]?` to make entire template sections optional
    - Example: `#[templatia(template = "[name={name}]?")]` omits `name=` entirely from output when `name` is `None`
    - Enables expressing presence/absence of optional sections while maintaining parse consistency

## Testing policy and documentation conventions
This repository follows AGENTS.md for documentation and testing conventions. In short:
- Documentation comments are written in English
- Examples aim to be minimal, correct, and preferably compilable as doctests
- Tests should reflect intended behavior; do not change tests merely to match an implementation if they already reflect the documented intent

## License
Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-ap.md) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-mit.md) or http://opensource.org/licenses/MIT)

You may use this software under the terms of either license.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Japanese README
[README_ja.md](README-ja.md)