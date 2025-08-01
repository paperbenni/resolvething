# Crush Agent Configuration

## Build Commands
- `cargo build` - Build the project
- `cargo build --release` - Build with optimizations

## Test Commands
- `cargo test` - Run all tests
- `cargo test --lib` - Run only library tests
- `cargo test <test_name>` - Run specific test by name
- `cargo test -- --nocapture` - Run tests with output captured

## Lint/Format Commands
- `cargo fmt` - Format code with rustfmt
- `cargo fmt --check` - Check if code is formatted correctly
- `cargo clippy` - Run clippy linter
- `cargo clippy --fix` - Automatically fix clippy issues

## Code Style Guidelines

### Imports
- Group imports in order: standard library, external crates, local modules
- Use `use` statements at the top of the file
- Avoid glob imports (`use std::io::*`) unless necessary

### Formatting
- Use `cargo fmt` for automatic formatting
- Max line width: 100 characters
- Use 4 spaces for indentation (no tabs)

### Types
- Prefer explicit type annotations for public APIs
- Use `impl Trait` for return types when appropriate
- Leverage Rust's type inference for local variables

### Naming Conventions
- Use snake_case for variables and functions
- Use PascalCase for types and traits
- Use UPPER_SNAKE_CASE for constants and statics
- Module names should be concise and clear

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Prefer `?` operator for error propagation
- Create custom error types when appropriate
- Use `anyhow` or `thiserror` for complex error handling

### Additional Notes
- Follow Rust best practices and idioms
- Write documentation for public APIs
- Keep functions focused and small
- Use iterators and functional programming patterns when appropriate