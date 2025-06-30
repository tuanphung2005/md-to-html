use super::inline::process_inline;

pub fn is_ordered_list_item(line: &str) -> bool {
    let trimmed = line.trim();
    if let Some(dot_pos) = trimmed.find('.') {
        if dot_pos > 0 && trimmed[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
            return trimmed.chars().nth(dot_pos + 1) == Some(' ');
        }
    }
    false
}

pub fn get_list_indent_level(line: &str) -> usize {
    line.chars().take_while(|&c| c == ' ').count()
}

pub fn process_unordered_list(lines: &[&str], start_idx: usize) -> (String, usize) {
    build_list(lines, start_idx, ListType::Unordered, 0)
}

pub fn process_ordered_list(lines: &[&str], start_idx: usize) -> (String, usize) {
    build_list(lines, start_idx, ListType::Ordered, 0)
}

#[derive(Clone, Copy)]
enum ListType {
    Unordered,
    Ordered,
}

fn build_list(lines: &[&str], start_idx: usize, list_type: ListType, depth: usize) -> (String, usize) {
    let base_indent = get_list_indent_level(lines[start_idx]);
    let indent = "  ".repeat(depth + 1);
    
    let mut html = match list_type {
        ListType::Unordered => format!("{}<ul>\n", "  ".repeat(depth)),
        ListType::Ordered => format!("{}<ol>\n", "  ".repeat(depth)),
    };
    
    let mut i = start_idx;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        
        let line_indent = get_list_indent_level(line);
        
        // end list if we reach a line with less indentation than the base
        if line_indent < base_indent {
            break;
        }
        
        // list item processing
        if line_indent == base_indent {
            if is_list_item(trimmed, list_type) {
                let item_text = extract_item_text(trimmed, list_type);
                html.push_str(&format!("{}<li>{}", indent, process_inline(item_text)));
                
                // nested content check
                if has_nested_content(lines, i, base_indent) {
                    html.push_str("\n");
                    let (nested_html, consumed) = process_nested_content(lines, i + 1, base_indent, depth + 1);
                    html.push_str(&nested_html);
                    html.push_str(&format!("{}  </li>\n", indent));
                    i += consumed;
                } else {
                    html.push_str("</li>\n");
                }
            } else {
                break;
            }
        } else if line_indent > base_indent {
            // this shouldn't happen if we handle nesting correctly
            break;
        }
        
        i += 1;
    }
    
    html.push_str(&format!("{}</{}>\n", "  ".repeat(depth), match list_type {
        ListType::Unordered => "ul",
        ListType::Ordered => "ol",
    }));
    
    (html, i - start_idx)
}

fn process_nested_content(lines: &[&str], start_idx: usize, parent_indent: usize, depth: usize) -> (String, usize) {
    let mut html = String::new();
    let mut i = start_idx;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        
        let line_indent = get_list_indent_level(line);
        
        // stop if we reach a line with less indentation than the parent
        if line_indent <= parent_indent {
            break;
        }
        
        // nested lists
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let (nested_html, consumed) = build_list(lines, i, ListType::Unordered, depth);
            html.push_str(&nested_html);
            i += consumed - 1;
        } else if is_ordered_list_item(trimmed) {
            let (nested_html, consumed) = build_list(lines, i, ListType::Ordered, depth);
            html.push_str(&nested_html);
            i += consumed - 1;
        } else {
            break;
        }
        
        i += 1;
    }
    
    (html, i - start_idx)
}

fn is_list_item(line: &str, list_type: ListType) -> bool {
    match list_type {
        ListType::Unordered => line.starts_with("- ") || line.starts_with("* "),
        ListType::Ordered => is_ordered_list_item(line),
    }
}

fn extract_item_text(line: &str, list_type: ListType) -> &str {
    match list_type {
        ListType::Unordered => &line[2..],
        ListType::Ordered => {
            if let Some(dot_pos) = line.find('.') {
                line[dot_pos + 1..].trim()
            } else {
                line
            }
        }
    }
}

fn has_nested_content(lines: &[&str], current_idx: usize, base_indent: usize) -> bool {
    if current_idx + 1 >= lines.len() {
        return false;
    }
    
    let next_line = lines[current_idx + 1];
    let next_indent = get_list_indent_level(next_line);
    let next_trimmed = next_line.trim();
    
    next_indent > base_indent && 
    (next_trimmed.starts_with("- ") || next_trimmed.starts_with("* ") || is_ordered_list_item(next_trimmed))
}
