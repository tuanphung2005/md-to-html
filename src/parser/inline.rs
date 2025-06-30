pub fn process_inline(text: &str) -> String {
    let mut result = text.to_string();
    result = process_inline_code(&result);
    result = process_images(&result);
    result = process_links(&result);
    result = process_bold(&result);
    result = process_italic(&result);

    result
}

pub fn process_inline_code(text: &str) -> String {
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

pub fn process_images(text: &str) -> String {
    let mut result = text.to_string();

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

pub fn process_links(text: &str) -> String {
    let mut result = text.to_string();
    
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

pub fn process_bold(text: &str) -> String {
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

pub fn process_italic(text: &str) -> String {
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
