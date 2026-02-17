# ADRScope Documentation

Welcome to the ADRScope documentation! This page helps you find the right documentation for your needs.

## 📚 Documentation by Type

Following the [Diátaxis framework](https://diataxis.fr/), our documentation is organized into four categories:

### 🎓 Tutorials (Learning-oriented)

Perfect for newcomers who want hands-on guidance:

- **[Getting Started](getting-started.md)** - Install ADRScope and generate your first viewer

### 🔧 How-to Guides (Problem-oriented)

Practical guides for specific tasks:

- **[User Guide](user-guide.md)** - Complete reference for all commands
- **[Library API Guide](library-api.md)** - Using ADRScope as a Rust library
- **[GitHub Action](../ACTION.md)** - CI/CD integration with GitHub Actions
- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions

### 📖 Reference (Information-oriented)

Precise technical descriptions:

- **[Configuration Reference](configuration.md)** - All options and settings
- **[Rust API Documentation](https://docs.rs/adrscope)** - Auto-generated API docs
- **[ADRScope Specification](spec/active/adrscope-spec_1.md)** - Formal specification

### 💡 Explanation (Understanding-oriented)

Background and decision rationale:

- **[Architecture Decision Records](decisions/)** - Design decisions and rationale
- **[Contributing Guide](../CONTRIBUTING.md)** - Development workflow and architecture

## 🚀 Quick Navigation

### New to ADRScope?

1. Read the [Getting Started Guide](getting-started.md)
2. Try the commands in the [User Guide](user-guide.md)
3. Check the [Configuration Reference](configuration.md) for customization

### Using ADRScope as a Library?

1. Read the [Library API Guide](library-api.md)
2. Browse the [Rust API Documentation](https://docs.rs/adrscope)
3. Check example code in the API guide

### Integrating with CI/CD?

1. Read the [GitHub Action Documentation](../ACTION.md)
2. See the [User Guide](user-guide.md) for validation options
3. Check [Troubleshooting](troubleshooting.md) for CI-specific issues

### Contributing to ADRScope?

1. Read the [Contributing Guide](../CONTRIBUTING.md)
2. Review our [Architecture Decision Records](decisions/)
3. Check the [Development](#development) section below

## 📦 Installation

### CLI Tool

````bash
# From crates.io
cargo install adrscope

# From source
git clone https://github.com/zircote/adrscope.git
cd adrscope
cargo install --path .
````

### Rust Library

````toml
[dependencies]
adrscope = "0.3"
````

### GitHub Action

````yaml
- uses: zircote/adrscope@v0
  with:
    command: validate
    input-dir: docs/decisions
````

## 🛠️ Development

### Prerequisites

- Rust 1.85+ (2024 edition)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) for supply chain security

### Quick Start

````bash
git clone https://github.com/zircote/adrscope.git
cd adrscope
make check  # Run fmt + lint + test
````

See the [Contributing Guide](../CONTRIBUTING.md) for detailed development workflow.

## 📄 Documentation Structure

````
docs/
├── README.md                    # This file - documentation index
├── getting-started.md           # Tutorial: first steps
├── user-guide.md                # How-to: CLI commands
├── configuration.md             # Reference: all options
├── library-api.md               # How-to: using as library
├── troubleshooting.md           # How-to: solve common issues
├── decisions/                   # Explanation: ADRs
│   ├── README.md
│   └── adr-*.md
└── spec/                        # Reference: formal spec
    └── active/
        └── adrscope-spec_1.md
````

## 🔗 External Resources

- **[GitHub Repository](https://github.com/zircote/adrscope)** - Source code and issue tracker
- **[Crates.io](https://crates.io/crates/adrscope)** - Published Rust crate
- **[docs.rs](https://docs.rs/adrscope)** - Auto-generated Rust documentation
- **[GitHub Marketplace](https://github.com/marketplace/actions/adrscope)** - GitHub Action listing
- **[Structured MADR](https://github.com/zircote/structured-madr)** - ADR format specification

## 🆘 Getting Help

- **Issues?** Check [Troubleshooting](troubleshooting.md)
- **Questions?** Open a [GitHub Discussion](https://github.com/zircote/adrscope/discussions)
- **Bugs?** File a [GitHub Issue](https://github.com/zircote/adrscope/issues)
- **Contributing?** See [CONTRIBUTING.md](../CONTRIBUTING.md)

## 📝 License

ADRScope is licensed under the [MIT License](../LICENSE).
