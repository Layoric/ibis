#![expect(clippy::unwrap_used)]

use ibis::frontend::markdown;
use pretty_assertions::assert_eq;

#[test]
fn test_basic_markdown() {
    let input = "# Heading 1\n## Heading 2\n\nParagraph with **bold** and *italic* text.";
    let expected = "<h2>Heading 1</h2>\n<h3>Heading 2</h3>\n<p>Paragraph with <strong>bold</strong> and <em>italic</em> text.</p>\n";
    assert_eq!(markdown::render_article_markdown(input), expected);
}

#[test]
fn test_links() {
    let input = "[Example](https://example.com)";
    let expected = "<p><a href=\"https://example.com\">Example</a></p>\n";
    assert_eq!(markdown::render_article_markdown(input), expected);
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
    let expected = r#"<p>Inline <span class="katex"><span class="katex-mathml"></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:0.68333em;vertical-align:0em;"></span><span class="mord mathnormal" style="margin-right:0.05764em;">E</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut" style="height:0.8141079999999999em;vertical-align:0em;"></span><span class="mord mathnormal">m</span><span class="mord"><span class="mord mathnormal">c</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8141079999999999em;"><span style="top:-3.063em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight">2</span></span></span></span></span></span></span></span></span></span></span> and display <span class="katex-display"><span class="katex"><span class="katex-mathml"></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1.384292em;vertical-align:-0.35582em;"></span><span class="mop"><span class="mop op-symbol large-op" style="margin-right:0.19445em;position:relative;top:-0.0005599999999999772em;">∫</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8592920000000001em;"><span style="top:-2.34418em;margin-left:-0.19445em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathnormal mtight">a</span></span></span><span style="top:-3.2579000000000002em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathnormal mtight">b</span></span></span></span><span class="vlist-s"></span></span><span class="vlist-r"><span class="vlist" style="height:0.35582em;"><span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.16666666666666666em;"></span><span class="mord mathnormal" style="margin-right:0.10764em;">f</span><span class="mopen">(</span><span class="mord mathnormal">x</span><span class="mclose">)</span><span class="mord mathnormal">d</span><span class="mord mathnormal">x</span></span></span></span></span></p>"#;
    assert_eq!(render_article_markdown(input), expected);
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
    let rendered = render_article_markdown(input);
    assert!(rendered.contains("Table of Contents"));
    assert!(rendered.contains("Heading 1"));
    assert!(rendered.contains("Subheading"));
    assert!(rendered.contains("Heading 2"));
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
