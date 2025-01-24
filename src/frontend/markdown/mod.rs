#![deny(clippy::unwrap_used)]

use article_link::ArticleLinkScanner;
use markdown_it::{
    plugins::cmark::block::{heading::ATXHeading, lheading::SetextHeader},
    MarkdownIt,
};
use math_equation::MathEquationScanner;
use std::sync::OnceLock;
use table_of_contents::{TocMarkerScanner, TocScanner};

pub mod article_link;
pub mod math_equation;
pub mod table_of_contents;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_markdown() {
        let input = "# Heading 1\n## Heading 2\n\nParagraph with **bold** and *italic* text.";
        let output = render_article_markdown(input);
        
        assert!(output.contains("Heading 1</h"));
        assert!(output.contains("Heading 2</h"));
        assert!(output.contains("<p>Paragraph with"));
        assert!(output.contains("<strong>bold</strong>"));
        assert!(output.contains("<em>italic</em>"));
    }

    #[test]
    fn test_links() {
        let input = "[Example](https://example.com)";
        let output = render_article_markdown(input);
        
        assert!(output.contains("<a href=\"https://example.com\">"));
        assert!(output.contains(">Example</a>"));
        assert!(output.contains("<p>") && output.contains("</p>"));
    }

    #[test]
    fn test_lists() {
        let input = "- Item 1\n- Item 2\n  1. Subitem";
        let expected = "<ul>\n<li>Item 1</li>\n<li>Item 2\n<ol>\n<li>Subitem</li>\n</ol>\n</li>\n</ul>\n";
        assert_eq!(render_article_markdown(input), expected);
    }

    #[test]
    fn test_code() {
        let input = "Inline `code` and\n```\nblock code\n```";
        let expected = "<p>Inline <code>code</code> and</p>\n<pre><code>block code\n</code></pre>\n";
        assert_eq!(render_article_markdown(input), expected);
    }

    #[test]
    fn test_tables() {
        let input = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        let expected = "<table>\n<thead>\n<tr>\n<th>Header 1</th>\n<th>Header 2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>Cell 1</td>\n<td>Cell 2</td>\n</tr>\n</tbody>\n</table>\n";
        assert_eq!(render_article_markdown(input), expected);
    }

    #[test]
    fn test_math_equations() {
        let input = r#"Inline $E=mc^2$ and display $$\int_a^b f(x)dx$$"#;
        let output = render_article_markdown(input);
        
        assert!(output.contains("katex"));
        assert!(output.contains("E=mc^2"));
        assert!(output.contains(r"\int_a^b f(x)dx"));
        assert!(output.contains("<p>") && output.contains("</p>"));
    }

    #[test]
    fn test_comment_markdown() {
        let input = "**Bold** and *italic* in comments";
        let expected = "<p><strong>Bold</strong> and <em>italic</em> in comments</p>\n";
        assert_eq!(render_comment_markdown(input), expected);
    }

    #[test]
    fn test_table_of_contents() {
        let input = r#"[!toc]
# Heading 1
## Subheading
# Heading 2"#;
        let rendered = render_article_markdown(input);
        
        assert!(rendered.contains("Table of Contents"));
        assert!(rendered.contains("Heading 1"));
        assert!(rendered.contains("Subheading")); 
        assert!(rendered.contains("Heading 2"));
        assert!(rendered.contains("href="));
        assert!(rendered.contains("not-prose"));
    }

    #[test]
    fn test_article_links() {
        let input = "[[Article@example.com]]";
        let output = render_article_markdown(input);
        
        assert!(output.contains(r#"href="/article/Article@example.com""#));
        assert!(output.contains(">Article</a>"));
        assert!(output.contains("<p>") && output.contains("</p>"));
    }

    #[test]
    fn test_spoilers() {
        let input = "::: spoiler\nHidden content\n:::";
        let output = render_article_markdown(input);
        
        println!("Spoiler test output:\n{}", output);
        
        assert!(output.contains("spoiler"), "Spoiler title missing");
        assert!(output.contains("Hidden content"), "Spoiler content missing");
        assert!(output.contains("<p>") && output.contains("</p>"), "Content not wrapped in paragraphs");
    }

    #[test]
    fn test_footnotes() {
        let input = "Text with footnote[^1]\n\n[^1]: Footnote content";
        let output = render_article_markdown(input);
        
        assert!(output.contains("Text with footnote"));
        assert!(output.contains("footnote-ref"));
        assert!(output.contains("footnotes"));
        assert!(output.contains("Footnote content"));
        assert!(output.contains("fn1"));
        assert!(output.contains("fnref1"));
    }
}

pub(crate) fn render_article_markdown(text: &str) -> String {
    static INSTANCE: OnceLock<MarkdownIt> = OnceLock::new();
    let mut parsed = INSTANCE.get_or_init(article_markdown).parse(text);

    // Make markdown headings one level smaller, so that h1 becomes h2 etc, and markdown titles
    // are smaller than page title.
    parsed.walk_mut(|node, _| {
        if let Some(heading) = node.cast_mut::<ATXHeading>() {
            heading.level += 1;
        }
        if let Some(heading) = node.cast_mut::<SetextHeader>() {
            heading.level += 1;
        }
    });
    parsed.render()
}

pub(crate) fn render_comment_markdown(text: &str) -> String {
    static INSTANCE: OnceLock<MarkdownIt> = OnceLock::new();
    INSTANCE.get_or_init(common_markdown).parse(text).render()
}

fn article_markdown() -> MarkdownIt {
    let mut parser = common_markdown();
    let p = &mut parser;

    // Extensions from various authors
    markdown_it_heading_anchors::add(p);
    markdown_it_block_spoiler::add(p);
    markdown_it_footnote::add(p);
    markdown_it_sub::add(p);
    markdown_it_sup::add(p);

    // Ibis custom extensions
    parser.inline.add_rule::<ArticleLinkScanner>();
    parser.inline.add_rule::<MathEquationScanner>();
    parser.inline.add_rule::<TocMarkerScanner>();
    parser.add_rule::<TocScanner>();

    parser
}

fn common_markdown() -> MarkdownIt {
    let mut parser = MarkdownIt::new();
    let p = &mut parser;
    {
        // Markdown-it inline core features. Image is disabled to prevent embedding external
        // images. Later we need to add proper image support using pictrs.
        use markdown_it::plugins::cmark::inline::*;
        newline::add(p);
        escape::add(p);
        backticks::add(p);
        emphasis::add(p);
        link::add(p);
        autolink::add(p);
        entity::add(p);
    }

    {
        // Markdown-it block core features. Unchanged from defaults.
        use markdown_it::plugins::cmark::block::*;
        code::add(p);
        fence::add(p);
        blockquote::add(p);
        hr::add(p);
        list::add(p);
        reference::add(p);
        heading::add(p);
        lheading::add(p);
        paragraph::add(p);
    }

    {
        // Some of the extras from markdown-it, others are intentionally excluded.
        use markdown_it::plugins::extra::*;
        strikethrough::add(p);
        tables::add(p);
        typographer::add(p);
    }

    parser
}
