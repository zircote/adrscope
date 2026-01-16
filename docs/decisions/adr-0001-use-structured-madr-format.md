---
title: Use Structured MADR Format for ADRs
description: Decision to adopt Structured MADR as the standard format for Architecture Decision Records in ADRScope
type: adr
category: documentation
tags:
  - adr
  - madr
  - documentation
  - standards
status: accepted
created: 2026-01-15
author: ADRScope Team
project: adrscope
technologies:
  - markdown
  - yaml
audience:
  - developers
  - architects
related: []
---

## Context

ADRScope needs a standardized format for its own Architecture Decision Records. The format should:

- Be well-documented and widely recognized
- Support structured metadata for tooling
- Enable faceted search and filtering
- Allow relationship tracking between decisions
- Be human-readable and easy to author

## Decision

We will use the **[zircote/structured-madr](https://github.com/zircote/structured-madr)** format for all ADRs in this project.

[Structured MADR](https://github.com/zircote/structured-madr) extends the standard MADR format with:

- YAML frontmatter for machine-readable metadata
- Required fields: `title`, `status`
- Recommended fields: `description`, `created`, `category`, `author`
- Optional fields: `tags`, `technologies`, `project`, `audience`, `related`
- Standard status values: `proposed`, `accepted`, `deprecated`, `superseded`

### Template Structure

```markdown
---
title: Short descriptive title
description: One-sentence summary
type: adr
category: architecture|api|security|...
tags: [tag1, tag2]
status: proposed|accepted|deprecated|superseded
created: YYYY-MM-DD
author: Team or person name
---

## Context
[Problem description]

## Decision
[What was decided]

## Consequences
[Positive and negative impacts]
```

## Consequences

### Positive

- Consistent format across all ADRs
- Full compatibility with ADRScope tooling
- Enables faceted search by status, category, tags, etc.
- Supports relationship graphs via `related` field
- Machine-readable metadata for automation

### Negative

- Slightly more verbose than minimal formats
- Requires YAML frontmatter knowledge
- More fields to maintain

### Neutral

- Aligns with industry-standard MADR practices
- Documents our own decision-making process
