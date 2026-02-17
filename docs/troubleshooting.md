# Troubleshooting

Common issues and solutions when using ADRScope.

## Installation Issues

### Rust Version Too Old

**Problem**: Build fails with edition or syntax errors.

**Solution**: Verify you have Rust 1.85 or later:

```bash
rustc --version
```

If needed, update Rust:

```bash
rustup update stable
```

### Build Fails with Network Error

**Problem**: `cargo install adrscope` fails with network timeout.

**Solution**: 
1. Check your internet connection
2. Try with cargo's offline mode if you have dependencies cached:
   ```bash
   cargo install adrscope --offline
   ```
3. Use a different crates.io mirror if available

### Permission Denied on Installation

**Problem**: `cargo install` fails with permission error.

**Solution**: Ensure `~/.cargo/bin` is writable, or install with:

```bash
cargo install --root ~/.local adrscope
```

Then add `~/.local/bin` to your PATH.

## Parsing Issues

### YAML Frontmatter Parse Error

**Problem**: Error message like "Parse error: invalid YAML frontmatter"

**Common Causes**:
1. Frontmatter delimiters (`---`) are missing or incorrect
2. Invalid YAML syntax
3. Special characters not properly escaped

**Solution**: Verify frontmatter format:

````markdown
---
title: My ADR Title
description: A description here
status: accepted
category: architecture
tags:
  - tag1
  - tag2
date: 2026-02-17
---

# My ADR Title

Content goes here...
````

**Debugging Steps**:

1. Check YAML syntax with a linter:
   ```bash
   # Extract frontmatter manually
   sed -n '/^---$/,/^---$/p' docs/decisions/adr-0001.md | yamllint -
   ```

2. Validate required fields are present:
   - `title` (required)
   - `status` (required)
   
3. Common YAML errors:
   ```yaml
   # ❌ Wrong - missing quotes for special characters
   title: Title: With Colons
   
   # ✅ Correct
   title: "Title: With Colons"
   
   # ❌ Wrong - invalid date format
   date: 02/17/2026
   
   # ✅ Correct
   date: 2026-02-17
   
   # ❌ Wrong - mixed indentation
   tags:
    - tag1
     - tag2
   
   # ✅ Correct
   tags:
     - tag1
     - tag2
   ```

### File Not Found

**Problem**: "No ADR files found in directory"

**Solution**:

1. Verify the input directory exists:
   ```bash
   ls -la docs/decisions
   ```

2. Check the glob pattern matches your files:
   ```bash
   # Default pattern
   adrscope generate --pattern "**/*.md"
   
   # Custom pattern for specific naming
   adrscope generate --pattern "adr-*.md"
   ```

3. Ensure files have `.md` extension

### Invalid Status Value

**Problem**: "Invalid status: xyz"

**Solution**: Status must be one of:
- `proposed`
- `accepted`
- `deprecated`
- `superseded`

Fix your frontmatter:
```yaml
# ❌ Wrong
status: approved

# ✅ Correct
status: accepted
```

## Validation Issues

### Validation Fails in Strict Mode

**Problem**: Validation passes normally but fails with `--strict`

**Explanation**: Strict mode treats warnings as errors.

**Solution**: Review all warnings and fix them, or run without `--strict`:

```bash
# See all warnings
adrscope validate

# Strict mode - warnings become errors
adrscope validate --strict
```

### Circular Dependency Detected

**Problem**: "Circular dependency detected: ADR-001 → ADR-002 → ADR-001"

**Solution**: Fix the `supersedes` relationships to form a directed acyclic graph:

```yaml
# ADR-001
superseded_by: ADR-002

# ADR-002 should NOT supersede ADR-001
# Remove or fix this:
supersedes:
  - ADR-001  # ❌ Creates a cycle
```

### Duplicate ADR Numbers

**Problem**: "Duplicate ADR number found: 0001"

**Solution**: Ensure each ADR has a unique number in its filename:

```bash
# ❌ Wrong - duplicate numbers
docs/decisions/adr-0001-first.md
docs/decisions/adr-0001-second.md

# ✅ Correct - unique numbers
docs/decisions/adr-0001-first.md
docs/decisions/adr-0002-second.md
```

## Generation Issues

### Output File Permission Denied

**Problem**: Cannot write to output file

**Solution**:

1. Check directory permissions:
   ```bash
   ls -la $(dirname output.html)
   ```

2. Use a writable location:
   ```bash
   adrscope generate --output ~/adr-viewer.html
   ```

### Generated HTML is Empty

**Problem**: HTML file is created but shows no ADRs

**Possible Causes**:
1. No ADR files matched the pattern
2. All ADRs failed to parse
3. Input directory is wrong

**Solution**:

1. Run with verbose mode:
   ```bash
   adrscope generate --verbose
   ```

2. Verify files are being found:
   ```bash
   find docs/decisions -name "*.md" -type f
   ```

3. Test parsing individual files:
   ```bash
   adrscope validate --verbose
   ```

### HTML File Too Large

**Problem**: Generated HTML is several MB

**Explanation**: ADRScope embeds all content, CSS, and JavaScript in a single HTML file.

**Solutions**:

1. This is expected for large ADR corpora (100+ ADRs)
2. Consider generating multiple viewers:
   ```bash
   # By status
   adrscope generate --output accepted.html --pattern "**/accepted/*.md"
   
   # By category
   adrscope generate --output security.html --pattern "**/*security*.md"
   ```

3. Use wiki format instead:
   ```bash
   adrscope wiki --input docs/decisions --output wiki/
   ```

## Wiki Generation Issues

### Wiki Directory Not Empty

**Problem**: Wiki output directory already has files

**Solution**: ADRScope will overwrite existing files. To be safe:

```bash
# Back up existing wiki
cp -r wiki wiki.backup

# Generate wiki
adrscope wiki --output wiki
```

### Wiki Links Not Working

**Problem**: Links between wiki pages are broken

**Explanation**: Ensure you're viewing the wiki in GitHub's wiki feature, not as raw Markdown.

**Solution**: 

1. Clone your wiki repository:
   ```bash
   git clone https://github.com/USERNAME/REPO.wiki.git
   cd REPO.wiki
   ```

2. Copy generated files:
   ```bash
   cp path/to/generated/wiki/*.md .
   ```

3. Commit and push:
   ```bash
   git add *.md
   git commit -m "Update ADR wiki pages"
   git push
   ```

## GitHub Action Issues

### Action Not Found

**Problem**: `uses: zircote/adrscope@v0` fails

**Solution**: Ensure you're using a valid version tag:

```yaml
- uses: zircote/adrscope@v0  # Latest v0.x
# or
- uses: zircote/adrscope@v0.3.0  # Specific version
```

### Validation Annotations Not Appearing

**Problem**: Errors don't show inline in PR

**Solution**: 

1. Ensure you're using the `validate` command
2. Check the action has write permissions:
   ```yaml
   permissions:
     contents: read
     pull-requests: write  # Required for annotations
   ```

3. Verify the action ran:
   ```yaml
   - name: Validate ADRs
     uses: zircote/adrscope@v0
     with:
       command: validate
       strict: true
   ```

### Workflow Not Triggering

**Problem**: Workflow doesn't run when ADRs change

**Solution**: Check your path filters:

```yaml
on:
  pull_request:
    paths:
      - 'docs/decisions/**'  # Must match your ADR directory
      - 'docs/adr/**'        # Add all relevant paths
```

## Performance Issues

### Slow Parsing

**Problem**: Takes a long time to parse ADRs

**Typical Performance**: 
- ~100 ADRs: <1 second
- ~1000 ADRs: <5 seconds
- ~10000 ADRs: <30 seconds

**If slower**:

1. Check for very large individual files (>1MB each)
2. Reduce glob pattern scope:
   ```bash
   # ❌ Slower - searches entire repo
   adrscope generate --pattern "**/*.md"
   
   # ✅ Faster - specific directory
   adrscope generate --input docs/decisions --pattern "*.md"
   ```

3. Use more specific patterns to exclude non-ADR files

### High Memory Usage

**Problem**: Process uses excessive memory

**Typical Memory Usage**:
- ~100 ADRs: <50 MB
- ~1000 ADRs: <200 MB

**If higher**:

1. Process ADRs in batches
2. Check for extremely large ADR files
3. Consider splitting your ADR corpus into multiple viewers

## Command-Line Issues

### Command Not Found

**Problem**: `adrscope: command not found`

**Solution**: Ensure `~/.cargo/bin` is in your PATH:

```bash
echo $PATH | grep cargo
```

If not present:

```bash
# For bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Help Not Showing

**Problem**: `adrscope --help` doesn't work

**Solution**: This should always work. If not:

1. Verify installation:
   ```bash
   which adrscope
   adrscope --version
   ```

2. Reinstall:
   ```bash
   cargo install --force adrscope
   ```

## Getting More Help

### Enable Verbose Mode

For any issue, try running with `--verbose`:

```bash
adrscope generate --verbose
adrscope validate --verbose
adrscope stats --verbose
adrscope wiki --verbose
```

This provides detailed debug information.

### Check Version

Ensure you're using the latest version:

```bash
adrscope --version
```

Update if needed:

```bash
cargo install adrscope --force
```

### Report a Bug

If you've found a bug:

1. Check [existing issues](https://github.com/zircote/adrscope/issues)
2. Create a new issue with:
   - ADRScope version (`adrscope --version`)
   - Operating system
   - Full error message
   - Minimal reproduction case
   - Sample ADR file (if relevant)

### Common Error Messages

| Error Message | Likely Cause | Solution |
|---------------|--------------|----------|
| "No ADR files found" | Wrong directory or pattern | Check path and glob pattern |
| "Parse error: invalid YAML" | Malformed frontmatter | Validate YAML syntax |
| "Invalid status" | Unknown status value | Use proposed/accepted/deprecated/superseded |
| "Permission denied" | No write access | Check file/directory permissions |
| "Circular dependency" | Invalid supersedes chain | Fix ADR relationships |
| "Duplicate ADR number" | Non-unique filenames | Rename files with unique numbers |

### Known Limitations

1. **Large Files**: ADRs over 10MB may parse slowly
2. **Unicode**: Full Unicode support, but some rare characters may render differently in HTML
3. **Windows Paths**: Use forward slashes in patterns even on Windows
4. **Glob Performance**: Extremely deep directory trees (>20 levels) may be slow

### Debug Checklist

When troubleshooting:

- [ ] Run with `--verbose` flag
- [ ] Check file permissions
- [ ] Verify YAML frontmatter format
- [ ] Test with a minimal ADR file
- [ ] Check glob pattern matches files
- [ ] Ensure Rust version is 1.85+
- [ ] Try latest version of ADRScope
- [ ] Review error message carefully
- [ ] Check existing GitHub issues

## Further Resources

- [User Guide](user-guide.md) - Complete command reference
- [Getting Started](getting-started.md) - Initial setup guide
- [Configuration](configuration.md) - All options explained
- [GitHub Issues](https://github.com/zircote/adrscope/issues) - Report bugs
- [GitHub Discussions](https://github.com/zircote/adrscope/discussions) - Ask questions
