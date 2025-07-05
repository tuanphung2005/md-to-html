pub mod blocks;
pub mod inline;
pub mod lists;
pub mod toc;
pub mod collapsible;
pub mod blockquote;

use crate::html::document::HtmlDocument;
use toc::TableOfContents;

pub fn markdown_to_html(markdown: &str, theme: Option<&str>) -> String {
    let mut doc = HtmlDocument::new();
    let mut toc = TableOfContents::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // headers and add to TOC
        if let Some(header) = blocks::process_header(line) {
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

    doc.to_html(&toc, theme)
}
