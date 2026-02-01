# Robin - BATMAN-adv Mesh Network Interface

## Project Overview

Robin is a Rust library and CLI tool for interacting with the BATMAN-adv kernel module. It provides a high-level interface to mesh network management, neighbor discovery, gateway configuration, and routing algorithm control. The project consists of:

- **Library** (`robin`): Core Rust API for netlink-based interaction with batman-adv
- **CLI Tool** (`robctl`): Command-line interface for mesh network operations

## Tech Stack

- **Language**: Rust Edition 2024
- **Core Dependencies** (refer to `Cargo.toml` files for exact versions):
  - `neli` (0.7) - Netlink protocol handling with async support
  - `tokio` (1.48) - Async runtime for I/O operations
  - `thiserror` (2.0) - Error handling
  - `clap` (4.5) - CLI argument parsing with derive macros
  - `macaddr` (1) - MAC address parsing and formatting
  - `bitflags` (2) - Type-safe bit flag operations
  - `comfy-table` (7) - CLI table formatting

## Project Structure

```
robin/
├── lib/           # Core library implementation (robin-lib)
│   └── src/
│       ├── client.rs      # High-level RobinClient API
│       ├── model.rs       # Data structures for mesh entities
│       ├── error.rs       # RobinError type
│       ├── netlink/       # Low-level netlink wrappers
│       └── commands/      # Batman-adv command implementations
├── cli/           # CLI tool implementation (robctl)
│   └── src/
│       ├── main.rs        # Entry point
│       ├── app.rs         # Command structure
│       └── *.rs           # Command handlers
└── .github/
    └── workflows/
        └── ci.yml         # CI pipeline
```

## Coding Standards

### General Rust Guidelines

- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `rustfmt` with default settings for formatting
- Adhere to Clippy lints with `-D warnings` (all warnings must be resolved)
- Prefer idiomatic Rust patterns and iterator chains over imperative loops
- Use `?` operator for error propagation instead of explicit unwrap/expect

### Documentation

- All public APIs must have Rustdoc comments with:
  - Brief description of functionality
  - Example usage where applicable
  - Parameter descriptions for non-obvious arguments
  - Error conditions that may occur
- Use `//!` for module-level documentation
- Use `///` for item-level documentation
- Use `//` for inline implementation comments only when logic is complex

### Async Programming

- All I/O operations must be async using Tokio
- Use `async fn` for public APIs that perform I/O
- Handle Tokio runtime lifecycle appropriately in CLI
- Prefer async/await syntax over manual future combinators

### Error Handling

- Use `thiserror` for deriving error types
- Return `Result<T, RobinError>` from fallible operations
- Provide descriptive error messages that help users diagnose issues
- Avoid panics in library code; reserve for truly unrecoverable situations
- In CLI, convert errors to user-friendly messages before display

### Netlink Operations

- Use the `neli` crate for all netlink communication
- Build generic netlink messages following batman-adv attribute conventions
- Parse responses using appropriate attribute extractors
- Handle missing/malformed attributes gracefully

## Testing Requirements

### Running Tests

```bash
cargo test --all          # Run all tests
cargo fmt -- --check      # Verify formatting
cargo clippy -- -D warnings  # Run linter
cargo tarpaulin --out Xml    # Generate coverage report
```

### Test Guidelines

- Write unit tests for all non-trivial functions
- Integration tests should validate end-to-end command flows
- Mock netlink responses where practical to avoid kernel dependencies
- Test error conditions and edge cases
- Aim for meaningful test coverage (CI tracks via tarpaulin)

### Testing Limitations

- Many operations require an actual batman-adv kernel module
- Tests may need to run with elevated privileges or be marked as ignored
- Document system requirements for integration tests

## Security Considerations

- Validate all MAC addresses and network identifiers before use
- Sanitize user input in CLI before passing to library
- Be cautious with netlink operations that modify kernel state
- Document operations that require elevated privileges
- Never log sensitive network information in production

## CLI Design Patterns

### Command Structure

- Use Clap derive macros for command definitions
- Organize commands by functional area (neighbors, gateways, settings, etc.)
- Provide consistent `--meshif` / `-m` flag for interface selection
- Use human-readable output with `comfy-table` for tabular data
- Support both long and short option names where appropriate

### Output Formatting

- Default to user-friendly table format for lists
- Support JSON output for machine-readable needs (if/when added)
- Include clear error messages with actionable suggestions
- Use appropriate exit codes (0 for success, non-zero for errors)

## API Design Principles

### Library Interface

- Keep the `RobinClient` API focused and discoverable
- Use strongly-typed models for mesh entities
- Return appropriate collection types (Vec, HashMap as needed)
- Make async nature explicit in function signatures
- Provide synchronous alternatives only if there's clear demand

### Naming Conventions

- Use full words, not abbreviations (except standard terms like "gw" for gateway)
- Method names should clearly indicate their action and subject
  - `get_*` for read operations
  - `set_*` for write operations
  - `create_*` / `destroy_*` for lifecycle operations
  - Plural names for list operations (e.g., `neighbors()`, `gateways()`)

## Internal Utilities and Patterns

### Netlink Attribute Handling

- Use helper functions in `commands/utils.rs` for common attribute operations
- Extract attributes by type using neli's typed getters
- Provide clear error messages when required attributes are missing

### Model Conversions

- Implement appropriate From/TryFrom traits for converting between raw and model types
- Use Display trait for human-readable representations
- Implement Debug for all public types

## Prohibited Practices

- Don't use `unwrap()` or `expect()` on operations that can fail in production code
- Avoid blocking I/O in async contexts
- Don't hardcode interface names or network addresses
- Avoid dependencies on external network services in core library
- Don't introduce breaking changes to public API without major version bump

## Common Operations Examples

### Adding a New Command

1. Define netlink message structure in `lib/src/commands/`
2. Implement response parsing
3. Add method to `RobinClient` in `lib/src/client.rs`
4. Create CLI handler in `cli/src/`
5. Wire up command in `cli/src/app.rs`
6. Add tests for both library and CLI
7. Update README with usage example

### Working with MAC Addresses

- Use the `macaddr` crate's `MacAddr` type
- Parse from strings using `MacAddr::from_str()`
- Format for display using Display trait
- Validate before use in netlink messages

## Contributing Guidelines

- Run `cargo fmt` before committing
- Ensure `cargo clippy -- -D warnings` passes
- Add tests for new functionality
- Update documentation for API changes
- Follow commit message conventions (use conventional commits if established)
- Test on Linux with batman-adv module available when possible

## Build and CI

The CI pipeline runs on GitHub Actions and performs:
1. Code formatting check (`cargo fmt -- --check`)
2. Linting with Clippy (`cargo clippy -- -D warnings`)
3. Test suite (`cargo test --all`)
4. Coverage reporting with tarpaulin

All checks must pass before merging.

## Cross-Platform Notes

- The library is Linux-specific due to batman-adv kernel module dependency
- CLI tool is designed for Linux environments
- Use appropriate cfg attributes for platform-specific code if needed

## Additional Resources

- [BATMAN-adv Documentation](https://www.open-mesh.org/projects/batman-adv/wiki)
- [Netlink Protocol](https://www.kernel.org/doc/html/latest/userspace-api/netlink/intro.html)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
