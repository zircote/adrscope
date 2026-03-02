# Documentation Index

Complete documentation for ADRScope.

## Getting Started

New to ADRScope? Start here:

1. **[Getting Started Guide](getting-started.md)** - Installation and first steps
2. **[User Guide](user-guide.md)** - Complete command reference
3. **[Configuration Reference](configuration.md)** - All configuration options

## Documentation by Type

### Tutorials (Learning-Oriented)

Step-by-step guides to learn ADRScope:

- **[Getting Started](getting-started.md)** - Your first ADR viewer
  - Installation
  - Creating your first viewer
  - Understanding the output

### How-To Guides (Problem-Oriented)

Practical guides for specific tasks:

- **[User Guide](user-guide.md)** - Common tasks and workflows
  - Generating HTML viewers
  - Validating ADRs
  - Computing statistics
  - Publishing to GitHub Wiki
  - Using as GitHub Action

- **[Troubleshooting](troubleshooting.md)** - Solving common problems
  - Installation issues
  - Parsing errors
  - Validation failures
  - GitHub Action problems

### Technical Reference (Information-Oriented)

Precise technical documentation:

- **[API Reference](api-reference.md)** - Complete API documentation
  - Application layer (use cases)
  - Domain layer (business logic)
  - Infrastructure layer (I/O, parsing, rendering)
  - Error handling
  - Examples

- **[Configuration Reference](configuration.md)** - All options explained
  - Command-line arguments
  - Pattern matching
  - Output formats
  - Theme options

- **[Dependencies Reference](dependencies.md)** - External crate dependencies
  - Core dependencies (clap, serde, askama, pulldown-cmark, etc.)
  - Development dependencies
  - Version constraints and update policy
  - Supply chain security

### Explanation (Understanding-Oriented)

Conceptual documentation and design decisions:

- **[Architecture](architecture.md)** - System design and structure
  - Clean Architecture layers
  - Design principles
  - Data flow
  - Testing strategy
  - Performance considerations

- **[ADR Index](decisions/README.md)** - Architecture decisions for this project
  - [ADR-0001: Use Structured MADR Format](decisions/adr-0001-use-structured-madr-format.md)
  - [ADR-0002: Clean Architecture Layers](decisions/adr-0002-clean-architecture-layers.md)
  - [ADR-0003: Trait-Based FileSystem Abstraction](decisions/adr-0003-trait-based-filesystem-abstraction.md)
  - [ADR-0004: Forbid Unsafe Code and Panics](decisions/adr-0004-forbid-unsafe-code-and-panics.md)
  - [ADR-0005: Unified Error Types with thiserror](decisions/adr-0005-unified-error-types-with-thiserror.md)
  - [ADR-0006: Cargo Deny Supply Chain Security](decisions/adr-0006-cargo-deny-supply-chain-security.md)
  - [ADR-0007: Askama Compile-Time Templates](decisions/adr-0007-askama-compile-time-templates.md)
  - [ADR-0008: Extensible Validation Rule Pattern](decisions/adr-0008-extensible-validation-rule-pattern.md)

## Documentation by Audience

### For End Users

If you want to use ADRScope as a command-line tool:

1. [Getting Started Guide](getting-started.md) - Install and run
2. [User Guide](user-guide.md) - Learn all commands
3. [Configuration Reference](configuration.md) - Customize behavior
4. [Troubleshooting](troubleshooting.md) - Fix common issues

### For GitHub Action Users

If you want to use ADRScope in CI/CD:

1. [User Guide: GitHub Action Section](user-guide.md#github-action) - Action setup
2. [README: GitHub Action Examples](../README.md#github-action) - Sample workflows
3. [Troubleshooting: GitHub Action Issues](troubleshooting.md#github-action-issues) - Common problems

### For Library Users

If you want to integrate ADRScope into your Rust application:

1. [API Reference](api-reference.md) - Complete API documentation
2. [Architecture](architecture.md) - Understand the design
3. [Contributing Guide](../CONTRIBUTING.md) - Development setup

### For Contributors

If you want to contribute to ADRScope:

1. [Contributing Guide](../CONTRIBUTING.md) - Development workflow
2. [Architecture](architecture.md) - Understand the codebase
3. [ADR Index](decisions/README.md) - Design decisions
4. [API Reference](api-reference.md) - API internals

## Quick Reference

### Common Tasks

| Task | Command | Documentation |
|------|---------|---------------|
| Generate HTML viewer | `adrscope generate` | [User Guide](user-guide.md#generate-command) |
| Validate ADRs | `adrscope validate` | [User Guide](user-guide.md#validate-command) |
| Show statistics | `adrscope stats` | [User Guide](user-guide.md#stats-command) |
| Generate wiki pages | `adrscope wiki` | [User Guide](user-guide.md#wiki-command) |
| Use in GitHub Actions | See workflow | [README](../README.md#github-action) |
| Use as Rust library | See examples | [API Reference](api-reference.md) |
| Check dependencies | `cargo tree` | [Dependencies](dependencies.md) |

### Common Issues

| Problem | Documentation |
|---------|---------------|
| Installation fails | [Troubleshooting: Installation](troubleshooting.md#installation-issues) |
| Parse errors | [Troubleshooting: Parsing](troubleshooting.md#parsing-issues) |
| Validation failures | [Troubleshooting: Validation](troubleshooting.md#validation-issues) |
| GitHub Action not working | [Troubleshooting: GitHub Action](troubleshooting.md#github-action-issues) |

## External Resources

- **[crates.io Package](https://crates.io/crates/adrscope)** - Install from Rust package registry
- **[docs.rs API Docs](https://docs.rs/adrscope)** - Auto-generated API documentation
- **[GitHub Repository](https://github.com/zircote/adrscope)** - Source code and issues
- **[GitHub Marketplace](https://github.com/marketplace/actions/adrscope)** - GitHub Action listing
- **[GitHub Discussions](https://github.com/zircote/adrscope/discussions)** - Community Q&A
- **[Structured MADR Format](https://github.com/zircote/structured-madr)** - ADR format spec

## Documentation Standards

This documentation follows:

- **[Diátaxis Framework](https://diataxis.fr/)** - Systematic documentation structure
- **[Google Developer Documentation Style Guide](https://developers.google.com/style)** - Writing style
- **[Microsoft Writing Style Guide](https://learn.microsoft.com/en-us/style-guide/welcome/)** - Terminology and tone

### Documentation Types

Following the Diátaxis methodology:

| Type | Purpose | Content |
|------|---------|---------|
| **Tutorials** | Learning-oriented | Step-by-step lessons for beginners |
| **How-To Guides** | Problem-oriented | Practical steps to solve specific problems |
| **Reference** | Information-oriented | Precise, technical descriptions |
| **Explanation** | Understanding-oriented | Conceptual clarification and discussion |

## Contributing to Documentation

Found an issue or want to improve the docs?

1. Check the [Contributing Guide](../CONTRIBUTING.md)
2. Documentation changes don't require full tests
3. Focus on clarity and accuracy
4. Use examples liberally
5. Follow the style guides above

### Documentation Source

All documentation is in Markdown format in the `docs/` directory:

```
docs/
├── README.md                    # This file
├── getting-started.md           # Tutorial
├── user-guide.md               # How-to guide
├── configuration.md            # Reference
├── api-reference.md            # Reference
├── architecture.md             # Explanation
├── troubleshooting.md          # How-to guide
├── decisions/                  # ADRs (explanation)
│   ├── README.md
│   └── adr-*.md
└── _assets/                    # Images and screenshots
    └── *.png
```

## Documentation Versioning

Documentation is versioned with the code:

- **Main branch** - Latest unreleased changes
- **Release tags** - Stable versioned documentation
- **docs.rs** - Auto-generated API docs per version

To view docs for a specific version:

```bash
# Clone repository
git clone https://github.com/zircote/adrscope.git
cd adrscope

# Checkout version
git checkout v0.3.0

# View docs
ls docs/
```

Or visit tagged releases on GitHub: https://github.com/zircote/adrscope/releases

## Getting Help

Can't find what you need?

1. **Search the documentation** - Use GitHub's search or your editor's search
2. **Check troubleshooting** - [Troubleshooting Guide](troubleshooting.md)
3. **Browse existing issues** - [GitHub Issues](https://github.com/zircote/adrscope/issues)
4. **Ask in discussions** - [GitHub Discussions](https://github.com/zircote/adrscope/discussions)
5. **Report missing docs** - [Open an issue](https://github.com/zircote/adrscope/issues/new)

---

**Last Updated**: 2026-02-24
**Version**: 0.3.0
