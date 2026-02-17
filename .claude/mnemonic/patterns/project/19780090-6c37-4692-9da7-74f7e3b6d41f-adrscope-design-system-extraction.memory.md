---
id: 19780090-6c37-4692-9da7-74f7e3b6d41f
type: semantic
namespace: patterns/project
created: 2026-01-25T21:15:00Z
modified: 2026-01-25T21:15:00Z
title: "ADRScope design system extraction - Slate palette, 4px grid, hybrid depth"
tags:
  - aesth
  - extracted
  - design-system
  - css
  - tokens
temporal:
  valid_from: 2026-01-25T21:15:00Z
  recorded_at: 2026-01-25T21:15:00Z
provenance:
  source_type: extraction
  agent: claude-opus-4
  confidence: 0.95
  source_file: templates/styles.css
---

# ADRScope Design System Extraction

Extracted design patterns from `templates/styles.css` (1215 lines, vanilla CSS, zero dependencies).

## Design Tokens

### Colors - Foundation (Slate Palette)

```css
/* Light Theme */
--color-bg: #ffffff;
--color-bg-secondary: #f8fafc;
--color-bg-tertiary: #f1f5f9;
--color-text: #1e293b;
--color-text-secondary: #64748b;
--color-text-muted: #94a3b8;
--color-border: #e2e8f0;
--color-border-hover: #cbd5e1;

/* Dark Theme */
--color-bg: #0f172a;
--color-bg-secondary: #1e293b;
--color-bg-tertiary: #334155;
--color-text: #f1f5f9;
--color-text-secondary: #94a3b8;
--color-text-muted: #64748b;
--color-border: #334155;
--color-border-hover: #475569;
```

### Colors - Accent (Blue)

```css
--color-primary: #3b82f6;        /* blue-500 */
--color-primary-hover: #2563eb;  /* blue-600 */
--color-primary-bg: #eff6ff;     /* blue-50 */

/* Dark */
--color-primary: #60a5fa;        /* blue-400 */
--color-primary-hover: #93c5fd;  /* blue-300 */
--color-primary-bg: #1e3a5f;
```

### Colors - Status

```css
--status-proposed: #f59e0b;      /* amber-500 */
--status-proposed-bg: #fef3c7;   /* amber-100 */
--status-accepted: #10b981;      /* emerald-500 */
--status-accepted-bg: #d1fae5;   /* emerald-100 */
--status-deprecated: #ef4444;    /* red-500 */
--status-deprecated-bg: #fee2e2; /* red-100 */
--status-superseded: #6b7280;    /* gray-500 */
--status-superseded-bg: #f3f4f6; /* gray-100 */
```

### Spacing Scale (4px base)

| Token | Value | Usage |
|-------|-------|-------|
| xs | 0.125rem (2px) | Badge padding |
| sm | 0.25rem (4px) | Chip gaps |
| md | 0.375rem (6px) | Tag gaps |
| base | 0.5rem (8px) | Standard padding |
| lg | 0.75rem (12px) | Section spacing |
| xl | 1rem (16px) | Component padding |
| 2xl | 1.5rem (24px) | Large spacing |
| 3xl | 2rem (32px) | Button sizes |

### Typography

```css
--font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
--font-mono: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;

/* Sizes */
body: 0.875rem (14px)
small: 0.75rem (12px)
tiny: 0.6875rem (11px)
h1: 1.5rem
h2: 1.25rem
h3: 1.125rem

/* Weights: 400, 500, 600, 700 */
/* Line-heights: 1.5 (body), 1.7 (prose) */
```

### Border Radius

```css
--border-radius: 6px;     /* Standard */
--border-radius-lg: 8px;  /* Cards, containers */
pill: 9999px;             /* Badges, chips */
small: 4px;               /* Code, kbd */
```

### Shadows

```css
--shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
--shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1);
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -4px rgba(0, 0, 0, 0.1);
```

### Transitions

```css
--transition-fast: 150ms ease;
--transition-normal: 200ms ease;
```

### Layout

```css
--sidebar-width: 280px;
--header-height: 60px;
--footer-height: 40px;
```

## Component Patterns

### Button

```css
.view-btn {
    width: 2rem;
    height: 2rem;
    border-radius: 4px;
    background: transparent;
    transition: background var(--transition-fast);
}

.theme-toggle {
    width: 2.25rem;
    height: 2.25rem;
    border: 1px solid var(--color-border);
    border-radius: var(--border-radius);
}
```

### Status Badge

```css
.status-badge {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 9999px;
    /* bg + color from status vars */
}
```

### Card

```css
.adr-card {
    padding: 1rem;
    border: 1px solid var(--color-border);
    border-radius: var(--border-radius-lg);
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}
.adr-card:hover {
    border-color: var(--color-border-hover);
    box-shadow: var(--shadow-md);
}
```

### Input

```css
input, select {
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: var(--border-radius);
    background: var(--color-bg);
}
input:focus {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-bg);
}
```

### Tag Chip

```css
.tag-chip {
    padding: 0.125rem 0.5rem;
    font-size: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 9999px;
}
```

## Inferred Design Direction

| Aspect | Value | Confidence |
|--------|-------|------------|
| Personality | Precision & Professional | High |
| Foundation | Cool Slate (Tailwind palette) | High |
| Depth Strategy | Hybrid (borders + shadows) | High |
| Typography | System fonts, technical | High |
| Spacing | 4px base grid | High |
| Theme Support | Full light/dark with auto | High |

## Rationale

Extracted via `/aesth:extract` to document existing design patterns for consistency. The design language aligns with developer tooling aesthetics (GitHub, VS Code, Tailwind UI). Zero external dependencies ensures the generated HTML viewer is fully self-contained.

## Relationships

- relates-to [[adrscope-architecture]]
- source [[templates/styles.css]]
