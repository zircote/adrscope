---
title: Adopt Clean Architecture with Domain/Application/Infrastructure Layers
description: Decision to structure the codebase using Clean Architecture principles with three distinct layers for separation of concerns
type: adr
category: architecture
tags:
  - architecture
  - clean-architecture
  - layers
  - separation-of-concerns
  - testability
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - rust
audience:
  - developers
  - architects
related: []
---

## Context

The project needed a way to organize code that would:

- Keep business logic testable in isolation
- Allow swapping infrastructure concerns (filesystem, parsers, renderers)
- Make the codebase navigable for new contributors

Without a clear architectural pattern, the codebase risked becoming a tangled mix of business logic and infrastructure concerns, making testing difficult and increasing coupling between components.

## Decision

We will adopt a **Clean Architecture** pattern with three distinct layers:

### Domain Layer (`domain/`)

Core business entities with no external dependencies:

- `Adr` - The central ADR entity
- `Frontmatter` - YAML frontmatter parsing and representation
- `Graph` - Relationship graph between ADRs
- `Validation` - Business rules and validation logic

The domain layer contains pure Rust types and logic. It has zero dependencies on filesystem, I/O, or external crates beyond standard library types.

### Application Layer (`application/`)

Use cases that orchestrate domain and infrastructure:

- `GenerateUseCase` - Generate HTML output from ADRs
- `ValidateUseCase` - Validate ADR correctness and consistency
- `WikiUseCase` - Generate wiki-compatible output
- `StatsUseCase` - Calculate statistics and metrics

The application layer depends on domain types and defines ports (traits) that infrastructure implements.

### Infrastructure Layer (`infrastructure/`)

External concerns and I/O:

- Filesystem operations (reading/writing ADR files)
- Markdown parsers
- HTML/template renderers
- Configuration loading

The infrastructure layer implements the ports defined by the application layer, allowing different implementations to be swapped.

### Dependency Rule

Dependencies flow inward only:

```
Infrastructure -> Application -> Domain
```

The domain layer has no dependencies on outer layers. The application layer depends only on domain. Infrastructure depends on both but is never depended upon by inner layers.

## Consequences

### Positive

- **Testable domain logic**: Business rules can be tested without mocking filesystem or I/O
- **Swappable infrastructure**: Parsers, renderers, and filesystem can be replaced without touching business logic
- **Clear boundaries**: New contributors can quickly understand where code belongs
- **Dependency inversion**: Application defines what it needs; infrastructure provides it

### Negative

- **More files and modules**: Three layers means more directory structure than a flat layout
- **Indirection**: Following the flow requires jumping between layers
- **Initial overhead**: Setting up the layer boundaries takes more effort upfront

### Trade-offs

The added complexity of multiple layers is justified by the gains in testability and maintainability. For a tool like ADRScope that needs to support multiple input formats, output formats, and filesystem operations, clean separation pays dividends as the codebase grows.
