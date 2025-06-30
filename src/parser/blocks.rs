use super::inline::process_inline;

pub struct Header {
    pub html: String,
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub fn process_header(line: &str) -> Option<Header> {
    let (level, content) = if line.starts_with("###### ") {
        (6, &line[7..])
    } else if line.starts_with("##### ") {
        (5, &line[6..])
    } else if line.starts_with("#### ") {
        (4, &line[5..])
    } else if line.starts_with("### ") {
        (3, &line[4..])
    } else if line.starts_with("## ") {
        (2, &line[3..])
    } else if line.starts_with("# ") {
        (1, &line[2..])
    } else {
        return None;
    };

    let processed_content = process_inline(content);
    let plain_text = strip_html_tags(&processed_content);
    let id = generate_id(&plain_text);
    
    let html = format!(
        "<h{} id=\"{}\">{}</h{}>\n",
        level, id, processed_content, level
    );
    
    Some(Header {
        html,
        level,
        text: plain_text,
        id,
    })
}

fn generate_id(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .trim_matches('_')
        .to_string()
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut inside_tag = false;
    
    for char in html.chars() {
        match char {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            c if !inside_tag => result.push(c),
            _ => {}
        }
    }
    
    result
}

pub fn process_paragraph(line: &str) -> String {
    format!("<p>{}</p>\n", process_inline(line))
}
