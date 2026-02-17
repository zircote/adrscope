# Troubleshooting

Common issues and solutions when using ADRScope.

## Installation Issues

### cargo install fails

**Problem:** Installation from crates.io fails with compilation errors.

**Solution:**

1. Ensure Rust 1.85 or later is installed:
   ````bash
   rustc --version
   ````

2. Update Rust if needed:
   ````bash
   rustup update stable
   ````

3. Try installing with verbose output to see the error:
   ````bash
   cargo install adrscope --verbose
   ````

### Binary not found after installation

**Problem:** `adrscope: command not found` after `cargo install`.

**Solution:**

Ensure `~/.cargo/bin` is in your PATH:

````bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"

# Then reload your shell
source ~/.bashrc
````

Verify installation:

````bash
which adrscope
adrscope --version
````

## Validation Issues

### "No ADR files found"

**Problem:** ADRScope can't find any ADR files.

**Solution:**

1. Verify the input directory exists:
   ````bash
   ls -la docs/decisions
   ````

2. Check the file pattern matches your files:
   ````bash
   # Default pattern is **/*.md
   adrscope validate --pattern "**/*.md" --verbose
   
   # Try a different pattern
   adrscope validate --pattern "adr-*.md"
   ````

3. Ensure files have the `.md` extension

### Validation fails with "Missing required field: title"

**Problem:** ADR is missing the required `title` field in frontmatter.

**Solution:**

Add a title to the YAML frontmatter:

````markdown
---
title: Your Decision Title Here
status: proposed
---
````

### "Unknown status value" warnings

**Problem:** ADRScope warns about unrecognized status values.

**Solution:**

ADRScope recognizes these status values:
- `proposed`
- `accepted`
- `deprecated`
- `superseded`

Unknown values are treated as `proposed` with a warning. To fix, use one of the recognized values:

````yaml
---
title: My Decision
status: accepted  # Use recognized value
---
````

### Validation passes but warnings appear

**Problem:** `adrscope validate` succeeds but shows warnings.

**Solution:**

Warnings indicate missing recommended fields. To make warnings fail in CI:

````bash
adrscope validate --strict
````

Add recommended fields to avoid warnings:

````yaml
---
title: Use PostgreSQL
description: Decision to use PostgreSQL as primary database
status: accepted
category: architecture
tags:
  - database
created: 2025-01-15
author: Architecture Team
---
````

## Parsing Issues

### YAML frontmatter parsing errors

**Problem:** `Parse error: invalid YAML` or similar.

**Solution:**

1. Ensure frontmatter is valid YAML between `---` markers:

   ````markdown
   ---
   title: My Decision
   status: accepted
   ---
   
   ## Context
   ````

2. Check for common YAML mistakes:
   - Missing quotes around special characters
   - Incorrect indentation
   - Missing colons

3. Validate YAML online: [YAML Lint](http://www.yamllint.com/)

### Markdown rendering issues

**Problem:** ADR content doesn't render correctly in HTML viewer.

**Solution:**

ADRScope uses CommonMark for markdown parsing. Ensure your markdown is valid:

1. Test with a markdown validator
2. Avoid non-standard markdown extensions
3. Use standard heading levels (`##`, `###`)

## Generation Issues

### HTML viewer is blank or empty

**Problem:** Generated HTML file opens but shows no ADRs.

**Solution:**

1. Check the console for JavaScript errors (F12 in browser)
2. Verify ADRs were parsed correctly:
   ````bash
   adrscope stats -i docs/decisions
   ````
3. Try regenerating with verbose output:
   ````bash
   adrscope generate --verbose
   ````

### Relationship graph doesn't show connections

**Problem:** ADR relationships aren't visible in the graph.

**Solution:**

1. Ensure `related` field uses correct filenames:
   ````yaml
   related:
     - adr-0001-first-decision.md
     - adr-0003-another-decision.md
   ````

2. Use exact filenames (case-sensitive)
3. Verify related ADRs exist in the same directory

### Large ADR collections are slow to load

**Problem:** Viewer is slow with 100+ ADRs.

**Solution:**

1. Split ADRs into multiple viewers by category:
   ````bash
   adrscope generate -i docs/decisions/architecture -o arch.html
   adrscope generate -i docs/decisions/security -o security.html
   ````

2. Use filtering to reduce the initial view

## Wiki Generation Issues

### Wiki pages have broken links

**Problem:** Links between wiki pages don't work.

**Solution:**

Ensure output directory structure is correct:

````bash
adrscope wiki -i docs/decisions -o wiki/
````

Generated files should be:
````
wiki/
├── ADR-Index.md
├── ADR-By-Status.md
├── ADR-By-Category.md
├── ADR-Timeline.md
└── ADR-Statistics.md
````

Deploy the entire `wiki/` directory to GitHub Wiki.

### GitHub Wiki deployment fails

**Problem:** Wiki action fails to deploy pages.

**Solution:**

1. Ensure the repository has Wiki enabled (Settings → Features → Wiki)

2. Check permissions in workflow:
   ````yaml
   permissions:
     contents: write
   ````

3. Verify the Wiki action configuration:
   ````yaml
   - uses: Andrew-Chen-Wang/github-wiki-action@v4
     with:
       path: wiki/
   ````

## GitHub Action Issues

### Action fails with "ADRScope binary not found"

**Problem:** GitHub Action can't find the adrscope binary.

**Solution:**

The action downloads pre-built binaries automatically. If this fails:

1. Check the runner's OS is supported (Linux, macOS, Windows)
2. Check the architecture is supported (x86_64, ARM64)
3. Try specifying a version:
   ````yaml
   - uses: zircote/adrscope@v0.3.0
   ````

### Annotations don't appear in PR

**Problem:** Validation errors don't show up as inline annotations.

**Solution:**

1. Ensure you're using the validate command:
   ````yaml
   - uses: zircote/adrscope@v0
     with:
       command: validate
   ````

2. Check the workflow has proper checkout:
   ````yaml
   - uses: actions/checkout@v4
   ````

3. Verify the PR includes changes to ADR files

### Action is slow on every run

**Problem:** Action takes a long time to download the binary.

**Solution:**

The action caches binaries automatically. If caching isn't working:

1. Check for cache permission issues
2. The first run will always be slower (downloads binary)
3. Subsequent runs should be ~2-5 seconds

## Library Usage Issues

### "InMemoryFileSystem not found" in tests

**Problem:** Can't use `InMemoryFileSystem` in tests.

**Solution:**

`InMemoryFileSystem` is in the `test_support` module:

````rust
#[cfg(test)]
use adrscope::infrastructure::fs::test_support::InMemoryFileSystem;

#[test]
fn my_test() {
    let fs = InMemoryFileSystem::new();
    // ...
}
````

It's automatically available in `#[cfg(test)]` mode.

### Compilation errors about missing types

**Problem:** Compiler can't find exported types.

**Solution:**

Ensure you're importing from the correct modules:

````rust
// Application layer
use adrscope::application::{GenerateUseCase, GenerateOptions};

// Domain types
use adrscope::domain::{Adr, Status, Frontmatter};

// Infrastructure
use adrscope::infrastructure::fs::RealFileSystem;

// Errors
use adrscope::{Error, Result};
````

## Performance Issues

### Validation is slow with many files

**Problem:** `adrscope validate` takes a long time.

**Solution:**

1. Use a more specific pattern to reduce file scanning:
   ````bash
   adrscope validate --pattern "adr-*.md"
   ````

2. Limit the directory depth:
   ````bash
   adrscope validate -i docs/decisions/active
   ````

3. Consider splitting large ADR collections

### HTML viewer loads slowly

**Problem:** Browser takes a long time to load the viewer.

**Solution:**

1. The viewer is a single self-contained HTML file. Large collections (500+ ADRs) may be slow.

2. Solutions:
   - Split into multiple viewers by category
   - Use pagination (feature request - contribute!)
   - Deploy to a web server instead of opening locally

## Platform-Specific Issues

### macOS: "unidentified developer" warning

**Problem:** macOS blocks the binary with a security warning.

**Solution:**

````bash
# Remove quarantine attribute
xattr -d com.apple.quarantine ~/.cargo/bin/adrscope

# Or install via Homebrew (signed)
brew install zircote/tap/adrscope
````

### Windows: Path issues with backslashes

**Problem:** File paths with backslashes don't work.

**Solution:**

Use forward slashes even on Windows:

````bash
adrscope generate -i docs/decisions -o output/viewer.html
````

Or use the glob pattern:

````bash
adrscope generate --pattern "**/*.md"
````

### Linux: Permission denied

**Problem:** `adrscope: permission denied` when running the binary.

**Solution:**

Make the binary executable:

````bash
chmod +x ~/.cargo/bin/adrscope
````

## Getting Help

If you encounter an issue not covered here:

1. **Search existing issues**: [GitHub Issues](https://github.com/zircote/adrscope/issues)
2. **Enable verbose output**: Add `--verbose` to see detailed logs
3. **Check versions**: Run `adrscope --version` and `rustc --version`
4. **Create an issue**: Include:
   - ADRScope version
   - Operating system
   - Rust version
   - Complete error message
   - Minimal reproduction steps

## Debug Mode

Run with verbose logging to troubleshoot:

````bash
# CLI usage
adrscope generate --verbose

# Library usage
env RUST_LOG=debug cargo run
````

## Common Workarounds

### Lenient parsing for non-standard ADRs

ADRScope is designed to be lenient. If you have ADRs from other tools:

1. Unknown status values default to `proposed`
2. Missing optional fields are handled gracefully
3. Extra frontmatter fields are ignored

### Converting from other ADR formats

If migrating from another ADR tool:

1. Keep the markdown body unchanged
2. Add YAML frontmatter with required fields:
   ````yaml
   ---
   title: [Extract from heading or filename]
   status: [Map to: proposed|accepted|deprecated|superseded]
   ---
   ````

3. Run validation to find missing fields:
   ````bash
   adrscope validate --strict
   ````

## See Also

- [Configuration Reference](configuration.md) - All options explained
- [User Guide](user-guide.md) - Complete command documentation
- [GitHub Issues](https://github.com/zircote/adrscope/issues) - Report bugs
- [GitHub Discussions](https://github.com/zircote/adrscope/discussions) - Ask questions
