---
title: Forbid Unsafe Code and Panic-Inducing Functions
description: Decision to enforce strict lints that forbid unsafe code blocks and deny panic-inducing functions in library code
type: adr
category: security, code-quality
tags:
  - safety
  - linting
  - error-handling
  - memory-safety
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - rust
  - clippy
audience:
  - developers
  - security-reviewers
related: []
---

## Context

Rust allows unsafe code blocks and panic-inducing functions (`unwrap`, `expect`, `panic!`). While these features provide flexibility and convenience, they present significant concerns for a library crate:

- **Unsafe code introduces memory safety risks**: The primary value proposition of Rust is memory safety guaranteed by the compiler. Unsafe blocks bypass these guarantees, requiring manual verification of correctness and opening the door to undefined behavior, data races, and memory corruption.

- **Panics crash the caller's application unexpectedly**: When library code panics, it unwinds the stack and terminates the calling application unless the panic is caught. This violates the principle of least surprise and removes error handling control from library consumers.

- **Both make auditing and trust harder**: Security auditors and downstream users must carefully examine unsafe blocks and potential panic sites. Eliminating these entirely makes the codebase easier to audit and increases confidence in the library's reliability.

For a library that may be used in critical systems, safety and predictability must take precedence over convenience.

## Decision

We will configure strict lints to forbid unsafe code and deny panic-inducing functions throughout the crate.

### Unsafe Code Prohibition

Add the following attribute to `lib.rs`:

```rust
#![forbid(unsafe_code)]
```

This uses `forbid` rather than `deny` to prevent any inner attributes from overriding this decision.

### Panic-Inducing Function Lints

Configure Clippy lints in `Cargo.toml` to deny panic-inducing patterns:

```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
```

### Error Handling Pattern

All fallible operations must return `Result` types, using the `?` operator for propagation:

```rust
// Correct approach
pub fn parse(input: &str) -> Result<Value, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    // ... parsing logic
    Ok(value)
}

// Forbidden approach
pub fn parse(input: &str) -> Value {
    input.parse().unwrap() // Lint error: unwrap_used
}
```

## Consequences

### Positive

- **Memory safety guaranteed by the compiler**: With `#![forbid(unsafe_code)]`, all code in the crate is verified safe by the Rust compiler. There are no escape hatches that could introduce undefined behavior.

- **All errors must be handled explicitly via `Result`**: Every potential failure point is visible in function signatures. Consumers of the library can handle errors appropriately for their use case rather than having their application crash.

- **Simplified security auditing**: Auditors do not need to examine unsafe blocks or trace potential panic paths. The codebase can be trusted to neither corrupt memory nor unexpectedly terminate.

- **Better API design**: Forcing explicit error handling encourages thoughtful API design where failure modes are documented and recoverable.

### Negative

- **More verbose error handling code**: Every fallible operation requires explicit handling. This increases line count and requires defining error types for all failure modes.

- **Cannot use certain performance optimizations requiring unsafe**: Some optimizations (e.g., unchecked indexing, manual memory management, SIMD intrinsics) require unsafe code. These are off-limits under this policy.

- **Some third-party APIs are harder to use**: External APIs that return `Option` or may panic require wrapper code to convert to `Result` types.

### Trade-off

This decision prioritizes **safety guarantees** over **flexibility and brevity**. The additional verbosity in error handling is an acceptable cost for the confidence that the library will never cause undefined behavior or crash the calling application. Projects that require unsafe optimizations should evaluate whether this library meets their performance requirements before adopting it.
