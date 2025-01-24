#![expect(clippy::unwrap_used)]

use ibis::frontend::markdown;
use pretty_assertions::assert_eq;
use ibis::frontend::markdown::render_article_markdown;

#[test]
fn test_basic_markdown() {
    let input = "# Heading 1\n## Heading 2\n\nParagraph with **bold** and *italic* text.";
    let output = markdown::render_article_markdown(input);
    
    // Check for presence of key elements
    assert!(output.contains("<h2>Heading 1</h2>"));
    assert!(output.contains("<h3>Heading 2</h3>"));
    assert!(output.contains("<p>Paragraph with <strong>bold</strong>"));
    assert!(output.contains("<em>italic</em> text.</p>"));
}

#[test]
fn test_links() {
    let input = "[Example](https://example.com)";
    let output = markdown::render_article_markdown(input);
    
    // Check for link with expected text and href
    assert!(output.contains("<a href=\"https://example.com\">"));
    assert!(output.contains(">Example</a>"));
    assert!(output.contains("<p>") && output.contains("</p>"));
}

#[test]
fn test_lists() {
    let input = "- Item 1\n- Item 2\n  1. Subitem";
    let expected = "<ul>\n<li>Item 1</li>\n<li>Item 2\n<ol>\n<li>Subitem</li>\n</ol>\n</li>\n</ul>\n";
    assert_eq!(markdown::render_article_markdown(input), expected);
}

#[test]
fn test_code() {
    let input = "Inline `code` and\n```\nblock code\n```";
    let expected = "<p>Inline <code>code</code> and</p>\n<pre><code>block code\n</code></pre>\n";
    assert_eq!(markdown::render_article_markdown(input), expected);
}

#[test]
fn test_tables() {
    let input = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let expected = "<table>\n<thead>\n<tr>\n<th>Header 1</th>\n<th>Header 2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>Cell 1</td>\n<td>Cell 2</td>\n</tr>\n</tbody>\n</table>\n";
    assert_eq!(markdown::render_article_markdown(input), expected);
}

#[test]
fn test_math_equations() {
    let input = r#"Inline $E=mc^2$ and display $$\int_a^b f(x)dx$$"#;
    let output = render_article_markdown(input);
    
    // Check for presence of math rendering
    assert!(output.contains("katex"));
    assert!(output.contains("E=mc^2"));
    assert!(output.contains(r"\int_a^b f(x)dx"));
    assert!(output.contains("<p>") && output.contains("</p>"));
}

#[test]
fn test_comment_markdown() {
    let input = "**Bold** and *italic* in comments";
    let expected = "<p><strong>Bold</strong> and <em>italic</em> in comments</p>\n";
    assert_eq!(markdown::render_comment_markdown(input), expected);
}

#[test]
fn test_table_of_contents() {
    let input = r#"[[toc]]
# Heading 1
## Subheading
# Heading 2"#;
    let rendered = markdown::render_article_markdown(input);
    
    // Check for TOC presence and structure
    assert!(rendered.contains("Table of Contents"));
    assert!(rendered.contains("<ul>") && rendered.contains("</ul>"));
    assert!(rendered.contains("Heading 1"));
    assert!(rendered.contains("Subheading"));
    assert!(rendered.contains("Heading 2"));
    assert!(rendered.contains("<a href="));
}

#[test]
fn test_article_links() {
    let input = "[[Article@example.com]]";
    let expected = r#"<p><a href="/article/Article@example.com">Article</a></p>"#;
    assert_eq!(render_article_markdown(input), expected);
}

#[test]
fn test_spoilers() {
    let input = "::: spoiler\nHidden content\n:::";
    let expected = "<details><summary>spoiler</summary>\n<p>Hidden content</p>\n</details>\n";
    assert_eq!(render_article_markdown(input), expected);
}

#[test]
fn test_footnotes() {
    let input = "Text with footnote[^1]\n\n[^1]: Footnote content";
    let expected = "<p>Text with footnote<sup class=\"footnote-ref\"><a href=\"#fn1\" id=\"fnref1\">1</a></sup></p>\n<section class=\"footnotes\">\n<ol>\n<li id=\"fn1\">\n<p>Footnote content <a href=\"#fnref1\" class=\"footnote-backref\">↩</a></p>\n</li>\n</ol>\n</section>\n";
    assert_eq!(render_article_markdown(input), expected);
}
