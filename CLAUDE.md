# CLAUDE.md - AI Assistant Guide for example-stripe-integration

## Project Overview

**Project Name:** example-stripe-integration
**Language:** Rust
**License:** MIT (Copyright 2025 Matthew Chenette)
**Purpose:** Example implementation of Stripe payment integration using only Rust's standard library

## Critical Constraints

### Standard Library Only
**IMPORTANT:** This project is designed to demonstrate Stripe integration using ONLY Rust's `std` library. Do NOT suggest or add any external crates/dependencies (including `reqwest`, `serde`, `tokio`, etc.).

All implementation must use:
- `std::net` for HTTP connections
- `std::io` for I/O operations
- `std::collections` for data structures
- Other `std::*` modules as needed

## Repository Structure

### Current State
This is a newly initialized repository with the following structure:

```
example-stripe-integration/
├── .github/
│   └── copilot-instructions.md  # AI coding assistant instructions
├── .git/                         # Git repository data
├── .gitignore                    # Rust-specific ignore patterns
├── LICENSE                       # MIT License
├── README.md                     # Project title
└── CLAUDE.md                     # This file
```

### Expected Structure (To Be Created)
The project will likely evolve to include:

```
example-stripe-integration/
├── src/
│   ├── main.rs                  # Entry point
│   ├── lib.rs                   # Library root (if applicable)
│   ├── stripe/                  # Stripe API integration modules
│   │   ├── mod.rs
│   │   ├── client.rs            # HTTP client implementation
│   │   ├── types.rs             # Stripe data structures
│   │   └── api/                 # API endpoints
│   ├── http/                    # Custom HTTP implementation
│   └── json/                    # Custom JSON parsing (if needed)
├── tests/                       # Integration tests
├── examples/                    # Usage examples
├── Cargo.toml                   # Project manifest
└── Cargo.lock                   # Dependency lockfile
```

## Development Workflow

### Initial Setup

When creating the initial Rust project structure:

1. Initialize with `cargo init` or manually create `Cargo.toml`
2. Set up minimal `src/main.rs` or `src/lib.rs`
3. Ensure no external dependencies are added to `Cargo.toml`
4. Follow Rust 2021 edition conventions

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run

# Check for errors without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Git Workflow

**Current Branch:** `claude/claude-md-migsr9t9coewgdeg-01RCGbZayb3MUzfy48zeBXAi`

When committing changes:
1. Make changes on the designated Claude branch
2. Write clear, descriptive commit messages
3. Push with: `git push -u origin <branch-name>`
4. Branch names must start with `claude/` and match the session ID

## Code Conventions

### Rust Style Guidelines

1. **Naming Conventions:**
   - Snake_case for functions, variables, and modules
   - CamelCase for types and traits
   - SCREAMING_SNAKE_CASE for constants

2. **Error Handling:**
   - Use `Result<T, E>` for recoverable errors
   - Use `panic!` sparingly, only for unrecoverable situations
   - Provide meaningful error messages
   - Consider custom error types using enums

3. **Documentation:**
   - Add doc comments (`///`) for public APIs
   - Include examples in doc comments where helpful
   - Document safety requirements for `unsafe` code

4. **Code Organization:**
   - Keep modules focused and cohesive
   - Use `mod.rs` for module organization
   - Separate concerns (HTTP, JSON parsing, API logic)

### Stripe Integration Specifics

When implementing Stripe API interactions:

1. **HTTP Handling:**
   - Implement HTTP client using `std::net::TcpStream`
   - Handle TLS manually or document limitations
   - Parse HTTP responses manually

2. **JSON Handling:**
   - Implement minimal JSON parser using `std::collections::HashMap`
   - Or manually parse JSON responses as needed
   - Handle Stripe's specific JSON structure

3. **API Design:**
   - Mirror Stripe's REST API structure
   - Support key Stripe objects: Charges, Customers, PaymentIntents, etc.
   - Implement proper authentication (API keys)

4. **Security:**
   - Never commit API keys or secrets
   - Add `.env` to `.gitignore` if using environment variables
   - Validate all inputs
   - Handle sensitive data appropriately

## Common Tasks for AI Assistants

### When Adding New Features

1. **Read existing code first:** Always use Read tool before modifying
2. **Check constraints:** Ensure no external dependencies are introduced
3. **Follow std-only pattern:** Implement features using standard library
4. **Test thoroughly:** Write tests for new functionality
5. **Document:** Add comments and documentation

### When Fixing Bugs

1. **Understand the issue:** Read relevant code sections
2. **Reproduce:** Ensure you understand the bug
3. **Fix minimally:** Make targeted changes
4. **Test:** Verify the fix works
5. **Commit:** Clear commit message describing the fix

### When Refactoring

1. **Justify the refactor:** Ensure it adds value
2. **Don't over-engineer:** Keep solutions simple
3. **Maintain compatibility:** Don't break existing APIs
4. **Test extensively:** Ensure no regressions

## Testing Strategy

### Unit Tests
- Test individual functions and modules
- Use `#[cfg(test)]` modules
- Mock Stripe responses where possible

### Integration Tests
- Place in `tests/` directory
- Test complete API workflows
- Consider using Stripe's test mode API keys (if making real requests)

### Example Tests
- Place working examples in `examples/` directory
- Each example should demonstrate a specific feature
- Include comments explaining the code

## Security Considerations

1. **API Key Management:**
   - Use environment variables for API keys
   - Never commit credentials
   - Document required environment variables

2. **Input Validation:**
   - Validate all user inputs
   - Sanitize data before sending to Stripe
   - Handle edge cases

3. **Error Messages:**
   - Don't leak sensitive information in errors
   - Log errors appropriately
   - Return user-friendly messages

## Debugging Tips

### Common Issues

1. **HTTP Parsing Errors:**
   - Manually implementing HTTP is error-prone
   - Test with simple requests first
   - Use `println!` debugging liberally

2. **JSON Parsing:**
   - Stripe's JSON can be complex
   - Start with simple objects
   - Handle nested structures carefully

3. **Connection Issues:**
   - Check network connectivity
   - Verify TLS/SSL handling
   - Test with curl first

## Performance Considerations

1. **Connection Pooling:**
   - Consider reusing TCP connections
   - Implement keepalive if needed

2. **Error Recovery:**
   - Implement retry logic for transient failures
   - Use exponential backoff

3. **Resource Cleanup:**
   - Ensure proper cleanup of network resources
   - Use RAII patterns

## Documentation Standards

### Code Comments
- Explain WHY, not WHAT
- Document assumptions
- Note limitations of std-only approach

### README Updates
When adding features, update README.md with:
- Installation instructions
- Usage examples
- Configuration requirements
- API coverage

## AI Assistant Workflow Checklist

Before completing any task:

- [ ] Read relevant existing code
- [ ] Verify no external dependencies added
- [ ] Follow Rust conventions
- [ ] Add appropriate tests
- [ ] Update documentation
- [ ] Run `cargo fmt` and `cargo clippy`
- [ ] Ensure code compiles (`cargo build`)
- [ ] Run tests (`cargo test`)
- [ ] Commit with clear message
- [ ] Push to correct branch

## Questions to Ask

When unclear about requirements:

1. Should this feature support all Stripe API versions or a specific one?
2. What level of error handling is expected?
3. Should we implement full HTTP/1.1 or minimal subset?
4. How should we handle TLS/HTTPS with std only?
5. What Stripe API endpoints are priority?

## Resources

### Stripe API Documentation
- [Stripe API Reference](https://stripe.com/docs/api)
- [Stripe Testing](https://stripe.com/docs/testing)

### Rust Documentation
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### HTTP/HTTPS with std
- [std::net module](https://doc.rust-lang.org/std/net/)
- [TcpStream](https://doc.rust-lang.org/std/net/struct.TcpStream.html)

## Version History

- **2025-11-27:** Initial CLAUDE.md creation - Repository structure documented
- Project is in initial setup phase
- No source code present yet

## Notes for Future Development

1. **TLS/HTTPS Challenge:** Implementing HTTPS without external crates is complex. Consider:
   - Documenting this limitation
   - Potentially using Stripe's test mode over HTTP (if available)
   - Or carefully implementing TLS subset needed

2. **JSON Parsing:** Manual JSON parsing is educational but error-prone:
   - Start with simple structures
   - Build up gradually
   - Consider a minimal JSON parser module

3. **API Coverage:** Start with core endpoints:
   - Payment Intents (recommended modern approach)
   - Charges (legacy but simpler)
   - Customers
   - Then expand based on needs

---

**Last Updated:** 2025-11-27
**Maintained for:** Claude and other AI coding assistants
**Repository Owner:** Matthew Chenette
