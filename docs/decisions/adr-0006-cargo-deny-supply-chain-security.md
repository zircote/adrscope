---
title: Use cargo-deny for Supply Chain Security
description: Decision to use cargo-deny for automated vulnerability scanning, license compliance, and source verification of dependencies
type: adr
category: security, tooling
tags:
  - security
  - supply-chain
  - dependencies
  - ci
  - compliance
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - cargo-deny
  - rust
audience:
  - developers
  - security-reviewers
  - maintainers
related:
  - adr-0004-forbid-unsafe-code-and-panics.md
---

## Context

Software supply chain attacks have become increasingly common, with dependencies introducing:

- **Known vulnerabilities**: Dependencies may contain CVEs that compromise application security. Without automated scanning, vulnerabilities can remain undetected in the dependency tree.

- **Licensing issues**: Transitive dependencies may introduce licenses incompatible with project requirements (e.g., copyleft licenses in a permissively licensed project). Manual license auditing is error-prone and does not scale.

- **Malicious code from untrusted sources**: Dependencies fetched from arbitrary git repositories or unknown registries may contain backdoors or malicious code. The Rust ecosystem defaults to crates.io, but Cargo allows alternative sources.

The project needs automated, enforceable checks that:

1. Scan dependencies against known CVE databases
2. Verify all licenses are permissive and compatible
3. Restrict dependency sources to trusted registries only
4. Block specific problematic crates

## Decision

We will use **cargo-deny** as the supply chain security tool, configured with strict policies.

### Configuration

The `deny.toml` configuration enforces:

#### Advisory Database

```toml
[advisories]
db-path = "~/.cargo/advisory-db"
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"
```

#### License Compliance

```toml
[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "BSL-1.0",
    "Unicode-3.0",
    "Unlicense",
    "Zlib",
]
copyleft = "deny"
```

Only permissive licenses are allowed. Copyleft licenses (GPL, LGPL, etc.) are denied to ensure the project can be used in any context without license inheritance requirements.

#### Banned Crates

```toml
[bans]
multiple-versions = "warn"
wildcards = "deny"
highlight = "all"

[[bans.deny]]
name = "openssl"
wrappers = []
```

The `openssl` crate is explicitly banned. Prefer `rustls` for TLS to:
- Avoid native library dependencies
- Reduce build complexity
- Eliminate OpenSSL-specific vulnerabilities

#### Source Verification

```toml
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

Only crates.io is allowed as a dependency source. Git dependencies and alternative registries are denied to ensure all code comes from the vetted, public registry.

### CI Integration

cargo-deny runs as part of the CI pipeline via `make ci`:

```bash
# Run all supply chain checks
cargo deny check

# Run specific checks
cargo deny check advisories
cargo deny check licenses
cargo deny check bans
cargo deny check sources
```

The CI pipeline fails if any check fails, preventing merges that introduce supply chain risks.

## Consequences

### Positive

- **Automated vulnerability detection in CI**: Every pull request and merge is scanned against the RustSec advisory database. Known vulnerabilities are caught before they reach production.

- **License compliance enforced**: The build fails if any dependency (direct or transitive) uses a non-approved license. This prevents accidental introduction of copyleft or proprietary licenses.

- **No dependencies from unknown sources**: By denying unknown registries and git sources, all dependencies must come from crates.io, which has accountability through crate ownership and yanking policies.

- **Explicit crate blocking**: Problematic crates like `openssl` can be explicitly banned, forcing the use of preferred alternatives.

### Negative

- **May block useful crates with incompatible licenses**: Some crates use licenses not in our allow-list (e.g., MPL-2.0). Adding these dependencies requires either adding the license to the allow-list or finding alternatives.

- **False positives from advisory database**: Occasionally, advisories may apply to configurations or features not used by the project, requiring exception annotations.

- **Additional CI time**: Running cargo-deny adds to CI pipeline duration, though typically only a few seconds.

### Trade-off

This decision prioritizes **security assurance** over **dependency flexibility**. The restrictions may occasionally require finding alternative crates or requesting license exceptions. This trade-off is acceptable because:

1. Supply chain attacks can have severe consequences
2. License compliance is a legal requirement
3. The Rust ecosystem has sufficient alternatives for most use cases
4. Exceptions can be documented and added when justified
