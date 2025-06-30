pub mod blocks;
pub mod inline;
pub mod lists;

use crate::html::document::HtmlDocument;

pub fn markdown_to_html(markdown: &str) -> String {
    let mut doc = HtmlDocument::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // headers
        if let Some(header) = blocks::process_header(line) {
            doc.add_content(&header);
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
        // paragraph or inline content
        else {
            let paragraph = blocks::process_paragraph(line);
            doc.add_content(&paragraph);
        }

        i += 1;
    }

    doc.to_html()
}
