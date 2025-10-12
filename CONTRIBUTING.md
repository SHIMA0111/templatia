# Contributing Guide

Thank you for your interest in contributing to this project!

## Development Environment Setup

### Requirements
- Rust 1.85 or later (using `rustup` is recommended)
- Git

### Setup Steps
```bash
# Clone the repository
git clone https://github.com/SHIMA0111/templatia.git
cd templatia

# Build
cargo build

# Run tests
cargo test

# Run lints
cargo clippy

# Check formatting
cargo fmt -- --check
```

## Contribution Workflow

### 1. Create or Find an Issue
- For bug fixes or new features, we recommend discussing in an issue first
- If working on an existing issue, comment to let others know

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Implement Your Changes
- Follow Rust coding conventions
- Add appropriate tests
- Include documentation comments (`///`)

### 4. Commit Your Changes
```bash
# Format code
cargo fmt

# Run lints
cargo clippy

# Run tests
cargo test

# Commit
git commit -m "Add: feature description" # example
```
If it is appropriate, please use commit messages that follow the [AGENTS.md](AGENTS.md)'s `Git Commit Messages` section.

### 5. Create a Pull Request
- Provide a clear title and description
- Fill out the PR template
- Ensure CI passes

## Coding Standards

### Style
- Use `cargo fmt` to format code
- Resolve all `cargo clippy` warnings

### Documentation
- Add doc comments to all public APIs
- Include usage examples where appropriate
- Verify documentation with `cargo doc --open`

### Testing
- Add tests for new features
- Add regression tests for bug fixes
- Use both unit tests and doc tests

### Error Handling
- Use appropriate error types
- Provide clear error messages

## Release Process

This project follows [Rust Semantic Versioning](https://doc.rust-lang.org/cargo/reference/semver.html).


## Code of Conduct

We expect all participants to be respectful and collaborative.

## License

Contributions will be licensed under the project's license (MIT/Apache-2.0, etc.).