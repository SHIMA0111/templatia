# Security Policy

## Supported Versions

As `templatia` is in its early stages of development (v0.0.x), only the latest version is actively supported with security updates. 
We encourage all users to stay on the most recent release.
In the security update will be released as the next 0.0.x version. 
And the previous version which reported the vulnerability will be marked as yanked on crates.io.

## Reporting a Vulnerability

We take all security vulnerabilities seriously. If you discover a security issue, please report it to us privately to protect the project and its users.

Please **do not** report security vulnerabilities through public GitHub issues.

Instead, please use the **[private vulnerability reporting feature](https://github.com/SHIMA0111/templatia/security/advisories/new)** provided by GitHub.

We will do our best to respond to your report promptly, acknowledge the issue, and work with you to address it. We kindly ask that you refrain from public disclosure until we have had a chance to release a fix.

## Security Practices

We have implemented the following measures to enhance the security of `templatia`:

### `unsafe` Code

The `templatia-derive` crate is built with `#![forbid(unsafe_code)]` to prevent the use of unsafe Rust code, leveraging Rust's memory safety guarantees to their fullest extent.

### Input Parsing

The core functionality of this library involves parsing user-defined templates and input strings. This is handled by the `chumsky` parser-combinator library, which is designed to safely handle complex parsing tasks. Malformed or unexpected input is designed to fail gracefully by returning a `TemplateError` rather than causing a panic or undefined behavior.

While the parser is robust, we acknowledge that highly complex templates or extremely large inputs could potentially lead to high resource consumption. Users handling untrusted input in production environments are advised to implement their own safeguards, such as input size limits or timeouts.

### Continuous Integration (CI)

Our CI pipeline automatically runs a suite of checks for every pull request, including:
-   **`cargo test`**: Ensures that all tests pass across multiple platforms (Linux, Windows, macOS).
-   **`cargo clippy`**: Lints the code to catch common mistakes and improve code quality.
-   **`cargo fmt`**: Checks for consistent code formatting.

We plan to incorporate `cargo audit` in the future to automatically scan for known vulnerabilities in our dependencies.

---
**Last Updated**: 2025-10-12