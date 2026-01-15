//! Markdown to HTML rendering.
//!
//! Uses pulldown-cmark for CommonMark-compliant markdown parsing.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, html};

/// Renders markdown content to HTML.
#[derive(Debug, Clone)]
pub struct MarkdownRenderer {
    options: Options,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownRenderer {
    /// Creates a new markdown renderer with default options.
    #[must_use]
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        Self { options }
    }

    /// Renders markdown content to HTML.
    #[must_use]
    pub fn render(&self, markdown: &str) -> String {
        let parser = Parser::new_ext(markdown, self.options);
        let mut html_output = String::with_capacity(markdown.len() * 2);
        html::push_html(&mut html_output, parser);
        html_output
    }

    /// Extracts plain text from markdown for search indexing.
    #[must_use]
    pub fn render_plain_text(&self, markdown: &str) -> String {
        let parser = Parser::new_ext(markdown, self.options);
        let mut text = String::with_capacity(markdown.len());
        let mut in_code_block = false;

        for event in parser {
            match event {
                Event::Text(t) | Event::Code(t) => {
                    if !in_code_block {
                        if !text.is_empty() && !text.ends_with(' ') {
                            text.push(' ');
                        }
                        text.push_str(&t);
                    }
                },
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                },
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                },
                Event::SoftBreak | Event::HardBreak => {
                    if !text.is_empty() && !text.ends_with(' ') {
                        text.push(' ');
                    }
                },
                _ => {},
            }
        }

        // Clean up whitespace
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("# Hello World");
        assert!(html.contains("<h1>Hello World</h1>"));
    }

    #[test]
    fn test_render_paragraph() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("This is a paragraph.");
        assert!(html.contains("<p>This is a paragraph.</p>"));
    }

    #[test]
    fn test_render_list() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("- Item 1\n- Item 2\n- Item 3");
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<li>Item 2</li>"));
        assert!(html.contains("<li>Item 3</li>"));
    }

    #[test]
    fn test_render_code_block() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("```rust\nfn main() {}\n```");
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_render_inline_code() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("Use `cargo build` to compile.");
        assert!(html.contains("<code>cargo build</code>"));
    }

    #[test]
    fn test_render_table() {
        let renderer = MarkdownRenderer::new();
        let md = r"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |";
        let html = renderer.render(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>"));
        assert!(html.contains("<td>"));
    }

    #[test]
    fn test_render_emphasis() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("This is *italic* and **bold** text.");
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_render_link() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("[Link text](https://example.com)");
        assert!(html.contains("<a href=\"https://example.com\">Link text</a>"));
    }

    #[test]
    fn test_render_strikethrough() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("This is ~~deleted~~ text.");
        assert!(html.contains("<del>deleted</del>"));
    }

    #[test]
    fn test_render_tasklist() {
        let renderer = MarkdownRenderer::new();
        let html = renderer.render("- [x] Done\n- [ ] Todo");
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("checked"));
    }

    #[test]
    fn test_plain_text_extraction() {
        let renderer = MarkdownRenderer::new();
        let md = r" Context

We need a **database** for our `application`.

## Decision

Use PostgreSQL.

```sql
SELECT * FROM users;
```

This is the end.";

        let text = renderer.render_plain_text(md);

        // Should contain text content
        assert!(text.contains("Context"));
        assert!(text.contains("database"));
        assert!(text.contains("application"));
        assert!(text.contains("Use PostgreSQL"));

        // Should NOT contain code blocks
        assert!(!text.contains("SELECT * FROM users"));

        // Should be clean without excessive whitespace
        assert!(!text.contains("  ")); // no double spaces
    }

    #[test]
    fn test_plain_text_basic() {
        let renderer = MarkdownRenderer::new();
        let text = renderer.render_plain_text("Hello **world**!");
        assert_eq!(text, "Hello world !");
    }

    #[test]
    fn test_plain_text_removes_formatting() {
        let renderer = MarkdownRenderer::new();
        let text = renderer.render_plain_text("This is *italic* and **bold**.");
        assert!(text.contains("italic"));
        assert!(text.contains("bold"));
        assert!(!text.contains("*"));
    }
}
