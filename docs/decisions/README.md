# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for ADRScope.

## ADR Index

| ID | Title | Status | Date |
|----|-------|--------|------|
| [ADR-0001](adr-0001-use-structured-madr-format.md) | Use Structured MADR Format | Accepted | 2026-01-15 |
| [ADR-0002](adr-0002-clean-architecture-layers.md) | Clean Architecture Layers | Accepted | 2026-01-15 |
| [ADR-0003](adr-0003-trait-based-filesystem-abstraction.md) | Trait-Based Filesystem Abstraction | Accepted | 2026-01-15 |
| [ADR-0004](adr-0004-forbid-unsafe-code-and-panics.md) | Forbid Unsafe Code and Panics | Accepted | 2026-01-15 |
| [ADR-0005](adr-0005-unified-error-types-with-thiserror.md) | Unified Error Types with thiserror | Accepted | 2026-01-15 |
| [ADR-0006](adr-0006-cargo-deny-supply-chain-security.md) | cargo-deny Supply Chain Security | Accepted | 2026-01-15 |
| [ADR-0007](adr-0007-askama-compile-time-templates.md) | Askama Compile-Time Templates | Accepted | 2026-01-15 |
| [ADR-0008](adr-0008-extensible-validation-rule-pattern.md) | Extensible Validation Rule Pattern | Accepted | 2026-01-15 |

## Status Legend

| Status | Description |
|--------|-------------|
| Proposed | Under discussion, not yet decided |
| Accepted | Decision has been made and is in effect |
| Deprecated | No longer recommended for new work |
| Superseded | Replaced by a newer decision |

## Categories

- **architecture** - Structural and organizational decisions
- **security** - Safety and supply chain decisions
- **tooling** - Build tools and development workflow
- **api** - Public interface design
- **testing** - Test strategy decisions

## Creating New ADRs

Use the Claude Code `/adr-new` command or create manually following the [zircote/structured-madr](https://github.com/zircote/structured-madr) template.

## Viewing ADRs

Generate an HTML viewer with faceted search:

```bash
adrscope generate -i docs/decisions -o adr-viewer.html
```
