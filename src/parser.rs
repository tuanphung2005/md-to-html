pub fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<title>Markdown to HTML</title>\n");
    html.push_str("<link rel=\"stylesheet\" href=\"styles.css\">\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // headers
        if line.starts_with("###### ") {
            html.push_str(&format!("<h6>{}</h6>\n", process_inline(&line[7..])));
        } else if line.starts_with("##### ") {
            html.push_str(&format!("<h5>{}</h5>\n", process_inline(&line[6..])));
        } else if line.starts_with("#### ") {
            html.push_str(&format!("<h4>{}</h4>\n", process_inline(&line[5..])));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", process_inline(&line[4..])));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", process_inline(&line[3..])));
        } else if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", process_inline(&line[2..])));
        }
        // unordered lists
        else if line.starts_with("- ") || line.starts_with("* ") {
            let (list_html, lines_consumed) = process_unordered_list(&lines, i);
            html.push_str(&list_html);
            i += lines_consumed - 1;
        }
        // ordered lists
        else if is_ordered_list_item(line) {
            let (list_html, lines_consumed) = process_ordered_list(&lines, i);
            html.push_str(&list_html);
            i += lines_consumed - 1;
        }
        // regular paragraph
        else {
            html.push_str(&format!("<p>{}</p>\n", process_inline(line)));
        }

        i += 1;
    }

    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

// process inline
fn process_inline(text: &str) -> String {
    let mut result = text.to_string();
    result = process_inline_code(&result);
    result = process_images(&result);
    result = process_links(&result);
    result = process_bold(&result);
    result = process_italic(&result);

    result
}

// ABOMINATIONS START HERE

fn process_inline_code(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    let mut in_code = false;

    while let Some(ch) = chars.next() {
        if ch == '`' {
            if in_code {
                result.push_str("</code>");
                in_code = false;
            } else {
                result.push_str("<code>");
                in_code = true;
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn process_images(text: &str) -> String {
    let mut result = text.to_string();

    // regex replacement for ![alt](url)
    while let Some(start) = result.find("![") {
        if let Some(alt_end) = result[start..].find("](") {
            let alt_start = start + 2;
            let alt_end = start + alt_end;
            
            if let Some(url_end) = result[alt_end..].find(')') {
                let url_start = alt_end + 2;
                let url_end = alt_end + url_end;
                
                let alt = &result[alt_start..alt_end];
                let url = &result[url_start..url_end];
                
                let img_tag = format!("<img src=\"{}\" alt=\"{}\">", url, alt);
                result = format!("{}{}{}", &result[..start], img_tag, &result[url_end + 1..]);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    result
}

fn process_links(text: &str) -> String {
    let mut result = text.to_string();
    
    // regex replacement for [text](url)
    while let Some(start) = result.find('[') {
        if let Some(text_end) = result[start..].find("](") {
            let text_start = start + 1;
            let text_end = start + text_end;
            
            if let Some(url_end) = result[text_end..].find(')') {
                let url_start = text_end + 2;
                let url_end = text_end + url_end;
                
                let link_text = &result[text_start..text_end];
                let url = &result[url_start..url_end];
                
                let link_tag = format!("<a href=\"{}\">{}</a>", url, link_text);
                result = format!("{}{}{}", &result[..start], link_tag, &result[url_end + 1..]);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    result
}

fn process_bold(text: &str) -> String {
    let mut result = text.to_string();
    
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let end = start + 2 + end;
            let bold_text = &result[start + 2..end];
            let bold_tag = format!("<strong>{}</strong>", bold_text);
            result = format!("{}{}{}", &result[..start], bold_tag, &result[end + 2..]);
        } else {
            break;
        }
    }
    
    result
}

// italic
fn process_italic(text: &str) -> String {
    let mut result = text.to_string();
    
    while let Some(start) = result.find('*') {

        if start > 0 && result.chars().nth(start - 1) == Some('*') {
            result = result[start + 1..].to_string();
            continue;
        }
        if start < result.len() - 1 && result.chars().nth(start + 1) == Some('*') {
            result = result[start + 2..].to_string();
            continue;
        }
        
        if let Some(end) = result[start + 1..].find('*') {
            let end = start + 1 + end;
            let italic_text = &result[start + 1..end];
            let italic_tag = format!("<em>{}</em>", italic_text);
            result = format!("{}{}{}", &result[..start], italic_tag, &result[end + 1..]);
        } else {
            break;
        }
    }
    
    result
}

fn is_ordered_list_item(line: &str) -> bool {
    let trimmed = line.trim();
    if let Some(dot_pos) = trimmed.find('.') {
        if dot_pos > 0 && trimmed[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
            return trimmed.chars().nth(dot_pos + 1) == Some(' ');
        }
    }
    false
}
// get indent lvl
fn get_list_indent_level(line: &str) -> usize {
    line.chars().take_while(|&c| c == ' ').count()
}

fn process_unordered_list(lines: &[&str], start_idx: usize) -> (String, usize) {
    let base_indent = get_list_indent_level(lines[start_idx]);
    let mut html = String::from("<ul>\n");
    let mut i = start_idx;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        
        let indent_level = get_list_indent_level(line);
        
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if indent_level == base_indent {

                let item_text = &trimmed[2..];
                html.push_str(&format!("  <li>{}</li>\n", process_inline(item_text)));
            } else if indent_level > base_indent {

                let (nested_html, lines_consumed) = process_nested_unordered_list(lines, i, base_indent);
                html.push_str(&nested_html);
                i += lines_consumed - 1;
            } else {
                // end of this list
                break;
            }
        } else if is_ordered_list_item(trimmed) && indent_level > base_indent {
            // nested ordered list in unordered list
            let (nested_html, lines_consumed) = process_nested_ordered_list(lines, i, base_indent);
            html.push_str(&nested_html);
            i += lines_consumed - 1;
        } else {
            // end of list
            break;
        }
        i += 1;
    }
    
    html.push_str("</ul>\n");
    (html, i - start_idx)
}

fn process_ordered_list(lines: &[&str], start_idx: usize) -> (String, usize) {
    let base_indent = get_list_indent_level(lines[start_idx]);
    let mut html = String::from("<ol>\n");
    let mut i = start_idx;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        
        let indent_level = get_list_indent_level(line);
        
        if is_ordered_list_item(trimmed) {
            if indent_level == base_indent {

                if let Some(dot_pos) = trimmed.find('.') {
                    let item_text = trimmed[dot_pos + 1..].trim();
                    html.push_str(&format!("  <li>{}</li>\n", process_inline(item_text)));
                }
            } else if indent_level > base_indent {
                // nested list
                let (nested_html, lines_consumed) = process_nested_ordered_list(lines, i, base_indent);
                html.push_str(&nested_html);
                i += lines_consumed - 1;
            } else {
                // end of this list
                break;
            }
        } else if (trimmed.starts_with("- ") || trimmed.starts_with("* ")) && indent_level > base_indent {
            // mixed nested unordered list in ordered list
            let (nested_html, lines_consumed) = process_nested_unordered_list(lines, i, base_indent);
            html.push_str(&nested_html);
            i += lines_consumed - 1;
        } else {
            // end
            break;
        }
        i += 1;
    }
    
    html.push_str("</ol>\n");
    (html, i - start_idx)
}

fn process_nested_unordered_list(lines: &[&str], start_idx: usize, parent_indent: usize) -> (String, usize) {
    let mut html = String::from("  <li>\n    <ul>\n");
    let base_indent = get_list_indent_level(lines[start_idx]);
    let mut i = start_idx;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        
        let indent_level = get_list_indent_level(line);
        
        if (trimmed.starts_with("- ") || trimmed.starts_with("* ")) && indent_level >= base_indent {
            if indent_level == base_indent {
                let item_text = &trimmed[2..];
                html.push_str(&format!("      <li>{}</li>\n", process_inline(item_text)));
            } else {
                // we go deeper
                let (deeper_nested, lines_consumed) = process_nested_unordered_list(lines, i, base_indent);
                html.push_str(&deeper_nested);
                i += lines_consumed - 1;
            }
        } else if indent_level <= parent_indent {

            break;
        } else {
            break;
        }
        i += 1;
    }
    // closes
    html.push_str("    </ul>\n  </li>\n");
    (html, i - start_idx)
}

fn process_nested_ordered_list(lines: &[&str], start_idx: usize, parent_indent: usize) -> (String, usize) {
    let mut html = String::from("  <li>\n    <ol>\n");
    let base_indent = get_list_indent_level(lines[start_idx]);
    let mut i = start_idx;
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        
        let indent_level = get_list_indent_level(line);
        
        if is_ordered_list_item(trimmed) && indent_level >= base_indent {
            if indent_level == base_indent {
                if let Some(dot_pos) = trimmed.find('.') {
                    let item_text = trimmed[dot_pos + 1..].trim();
                    html.push_str(&format!("      <li>{}</li>\n", process_inline(item_text)));
                }
            } else {
                // we go deeper
                let (deeper_nested, lines_consumed) = process_nested_ordered_list(lines, i, base_indent);
                html.push_str(&deeper_nested);
                i += lines_consumed - 1;
            }
        } else if indent_level <= parent_indent {
            // end of this list
            break;
        } else {
            break;
        }
        i += 1;
    }
    // closes
    html.push_str("    </ol>\n  </li>\n");
    (html, i - start_idx)
}