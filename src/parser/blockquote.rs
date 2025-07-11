use super::inline::process_inline;

pub fn process_blockquote(lines: &[&str], start_idx: usize) -> (String, usize) {
    let mut html = String::from("<blockquote>\n");
    let mut i = start_idx;
    let mut current_paragraph = String::new();
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.starts_with("> ") {

            let content = &trimmed[2..];
            
            if content.is_empty() {

                if !current_paragraph.is_empty() {
                    html.push_str(&format!("  <p>{}</p>\n", process_inline(&current_paragraph.trim())));
                    current_paragraph.clear();
                }

            } else {

                if !current_paragraph.is_empty() {
                    current_paragraph.push(' ');
                }
                
                current_paragraph.push_str(content);
            }
        } else if trimmed.starts_with(">") && trimmed.len() == 1 {
            // empty blockquote
            if !current_paragraph.is_empty() {
                html.push_str(&format!("  <p>{}</p>\n", process_inline(&current_paragraph.trim())));
                current_paragraph.clear();
            }
        } else {

            break;
        }
        
        i += 1;
    }
    
    // p content
    if !current_paragraph.is_empty() {
        html.push_str(&format!("  <p>{}</p>\n", process_inline(&current_paragraph.trim())));
    }
    
    html.push_str("</blockquote>\n");
    (html, i - start_idx)
}

pub fn is_blockquote_start(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("> ") || (trimmed == ">")
}
