use super::inline::process_inline;

pub fn process_collapsible_block(lines: &[&str], start_idx: usize) -> Option<(String, usize)> {
    let start_line = lines[start_idx].trim();
    
    // detail blocks
    if !start_line.starts_with("<details") {
        return None;
    }
    
    let mut html = String::new();
    let mut i = start_idx;
    let mut found_summary = false;
    let mut summary_content = String::new();
    let mut details_content = String::new();
    let mut in_summary = false;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.starts_with("<details") {
            html.push_str("<details>\n");
        } else if trimmed.starts_with("<summary>") {
            // summary content
            if let Some(end_pos) = trimmed.find("</summary>") {
                // summary line
                let content = &trimmed[9..end_pos];
                summary_content = process_inline(content);
                found_summary = true;
            } else {
                // multi-line summary
                let content = &trimmed[9..];
                summary_content.push_str(&process_inline(content));
                in_summary = true;
            }
        } else if trimmed.ends_with("</summary>") && in_summary {

            // end of multi-line summary
            let content = &trimmed[..trimmed.len() - 10];
            summary_content.push_str(&process_inline(content));
            in_summary = false;
            found_summary = true;

        } else if in_summary {

            // content inside multi-line summary
            summary_content.push_str(&process_inline(trimmed));
            summary_content.push(' ');

        } else if trimmed.starts_with("</details>") {

            // end of details block
            html.push_str(&format!("  <summary>{}</summary>\n", summary_content.trim()));
            if !details_content.trim().is_empty() {
                html.push_str(&format!("  <div class=\"details-content\">\n    {}\n  </div>\n", details_content.trim()));
            }
            html.push_str("</details>\n");
            return Some((html, i - start_idx + 1));
        } else if found_summary && !trimmed.is_empty() {
            // content inside details
            details_content.push_str(&format!("<p>{}</p>\n    ", process_inline(trimmed)));
        }
        
        i += 1;
    }
    None
}

pub fn is_collapsible_start(line: &str) -> bool {
    line.trim().starts_with("<details")
}