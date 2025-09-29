# templatia

A library for turning structs into simple text templates and parsing them back. It consists of:

- templatia: the core trait and error type
- templatia-derive: a procedural macro that implements the trait for your struct

Both crates are part of this repository. Most users will depend only on templatia with the derive feature enabled.

## Features
- Derive Template for named structs
- Auto-generate a default template (one field per line: `name = {name}`)
- Optional custom template via attribute: `#[templatia(template = "...")]`
- Round-trip support: `to_string` and `from_string`
- Clear error reporting (`TemplateError`)

## Minimum supported Rust version (MSRV)
- Rust 1.85.0
- Edition 2024

## Installation
Add templatia to your Cargo.toml. You can either:

1) Use the derive feature and import everything from templatia

```toml
[dependencies]
templatia = { version = "0.0.1", features = ["derive"] }
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
    let s = cfg.to_string();
    assert!(s.contains("host = localhost"));
    assert!(s.contains("port = 5432"));
}
```


## Usage
### Default template
When no template is specified, a default template is synthesized with one field per line:

```text
name = {name}
port = {port}
```

### Custom template
Provide a custom format using the templatia attribute and placeholder names that match your struct fields:

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
    assert_eq!(cfg.to_string(), "db.example.com:3306");

    let parsed = DbCfg::from_string("db.example.com:3306").unwrap();
    assert_eq!(parsed.host, "db.example.com");
    assert_eq!(parsed.port, 3306);
}
```

### Placeholders and types
- Each `{name}` in the template must correspond to a named struct field
- Field types used in the template must implement Display and FromStr
- Duplicate placeholders are allowed, but values must be consistent if they appear more than once

## Errors
templatia defines a simple error type for parsing and validation:

- TemplateError::InconsistentValues { placeholder, first_value, second_value }
  - Emitted when the same placeholder appears multiple times with conflicting parsed values
- TemplateError::Parse(String)
  - Generic parse error message

## Crates overview
- templatia
  - Template trait with two methods: `fn to_string(&self) -> String` and `fn from_string(s: &str) -> Result<Self::Struct, Self::Error>`
  - TemplateError enum for error reporting
- templatia-derive
  - #[derive(Template)] macro for named structs
  - Optional attribute: `#[templatia(template = "...")]`
  - Validates that placeholders exist as fields

## Feature flags
- derive
  - Enables the re-export of the proc-macro and the internal parser dependency needed for parsing from strings

## Load Map (0.0.x roadmap toward 0.1.0)
- 0.0.2
  - [ ] Emit warnings when not all struct fields are present in the template during parsing
  - [ ] Define default behavior for missing data: decide behavior when some or all fields are not included in the template
  - [ ] Option<T>: default to `None` when the placeholder is absent
  - [ ] String: add configurable handler for missing placeholders (error vs `String::new()`)
- 0.0.3
  - [ ] Enrich error handling and warnings (clearer diagnostics and coverage)
- 0.0.4
  - [ ] Declarative templates for field collections such as `Vec`, `HashMap`, and `HashSet`
  - [ ] Add `container` attribute to increase flexibility at the parent structure level
- 0.0.5
  - [ ] Support additional data forms: tuple (unnamed) structs, union structs, and enums

## Testing policy and documentation conventions
This repository follows AGENTS.md for documentation and testing conventions. In short:
- Documentation comments are written in English
- Examples aim to be minimal, correct, and preferably compilable as doctests
- Tests should reflect intended behavior; do not change tests merely to match an implementation if they already reflect the documented intent

## License
Dual-licensed under either of:
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

You may use this software under the terms of either license.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Japanese README
[README_ja.md](README-ja.md)