pub mod blocks;
pub mod inline;
pub mod lists;
pub mod toc;
pub mod collapsible;
pub mod blockquote;
pub mod codeblock;
pub mod footnote;

use crate::html::document::HtmlDocument;
use toc::TableOfContents;
use footnote::FootnoteManager;

pub fn markdown_to_html(markdown: &str, theme: Option<&str>) -> String {
    let mut doc = HtmlDocument::new();
    let mut toc = TableOfContents::new();
    let mut footnote_manager = FootnoteManager::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // footnote definitions
        if footnote::is_footnote_definition(line) {
            if let Some((id, content)) = footnote::parse_footnote_definition(line) {
                footnote_manager.add_footnote(id, content);
            }
        }
        // headers and add to TOC
        else if let Some(header) = blocks::process_header(line) {
            toc.add_header(&header);
            doc.add_content(&header.html);
        }
        // unordered lists
        else if line.starts_with("- ") || line.starts_with("* ") {
            let (list_html, lines_consumed) = lists::process_unordered_list(&lines, i);
            doc.add_content(&list_html);
            i += lines_consumed - 1;
        }
        // ordered lists
        else if lists::is_ordered_list_item(line) {
            let (list_html, lines_consumed) = lists::process_ordered_list(&lines, i);
            doc.add_content(&list_html);
            i += lines_consumed - 1;
        }

        // collapsible sections
        else if collapsible::is_collapsible_start(line) {
            if let Some((collapsible_html, lines_consumed)) = collapsible::process_collapsible_block(&lines, i) {
                doc.add_content(&collapsible_html);
                i += lines_consumed - 1;
            } else {
                // treat as regular paragraph if collapsible parsing fails
                let paragraph = blocks::process_paragraph(line);
                doc.add_content(&paragraph);
            }
        }
        
        // code blocks
        else if codeblock::is_code_block_start(line) {
            if let Some((code_html, lines_consumed)) = codeblock::process_code_block(&lines, i) {
                doc.add_content(&code_html);
                i += lines_consumed - 1;
            } else {
                // treat as regular paragraph if code block parsing fails
                let paragraph = blocks::process_paragraph(line);
                doc.add_content(&paragraph);
            }
        }
        
        // blockquotes
        else if blockquote::is_blockquote_start(line) {
            let (blockquote_html, lines_consumed) = blockquote::process_blockquote(&lines, i);
            doc.add_content(&blockquote_html);
            i += lines_consumed - 1;
        }

        // more feats here
        // ...

        // paragraph or inline content
        else {
            let paragraph = blocks::process_paragraph(line);
            doc.add_content(&paragraph);
        }

        i += 1;
    }

    // Process footnote references in content and add footnotes section
    if footnote_manager.has_footnotes() {
        let current_content = doc.get_content();
        let content_with_footnotes = footnote_manager.process_footnote_references(&current_content);
        doc.set_content(content_with_footnotes);
        
        let footnote_section = footnote_manager.generate_footnote_section();
        doc.add_content(&footnote_section);
    }

    doc.to_html(&toc, theme)
}
