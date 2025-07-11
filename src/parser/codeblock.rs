pub fn process_code_block(lines: &[&str], start_idx: usize) -> Option<(String, usize)> {
    let start_line = lines[start_idx].trim();
    if !start_line.starts_with("```") {
        return None;
    }

    // extract language
    let language = start_line[3..].trim();
    
    let mut code_content = String::new();
    let mut i = start_idx + 1;

    // finding ```
    while i < lines.len() {
        let line = lines[i];
        if line.trim() == "```" {
            // found closing tag, return the code block
            let html = if language.is_empty() {
                format!("<pre><code>{}</code></pre>\n", escape_html(&code_content))
            } else {
                format!("<pre><code class=\"language-{}\">{}</code></pre>\n", language, escape_html(&code_content))
            };
            return Some((html, i - start_idx + 1));
        }
        
        // line to code content
        if !code_content.is_empty() {
            code_content.push('\n');
        }
        code_content.push_str(line);
        i += 1;
    }

    // none as regular paragraph if no closing tag found
    None
}

pub fn is_code_block_start(line: &str) -> bool {
    line.trim().starts_with("```")
}

fn escape_html(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
