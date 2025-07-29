use super::inline::process_inline;
use std::collections::HashMap;

pub struct FootnoteManager {
    footnotes: HashMap<String, String>,
    footnote_order: Vec<String>,
}

impl FootnoteManager {
    pub fn new() -> Self {
        Self {
            footnotes: HashMap::new(),
            footnote_order: Vec::new(),
        }
    }

    pub fn add_footnote(&mut self, id: String, content: String) {
        if !self.footnotes.contains_key(&id) {
            self.footnote_order.push(id.clone());
        }
        self.footnotes.insert(id, content);
    }

    pub fn has_footnotes(&self) -> bool {
        !self.footnotes.is_empty()
    }

    pub fn generate_footnote_section(&self) -> String {
        if self.footnotes.is_empty() {
            return String::new();
        }

        let mut html = String::new();
        html.push_str("<div class=\"footnotes\">\n");
        html.push_str("  <ol>\n");

        for id in &self.footnote_order {
            if let Some(content) = self.footnotes.get(id) {
                html.push_str(&format!(
                    "    <li id=\"fn-{}\"><p>{} <a href=\"#fnref-{}\" class=\"footnote-backref\">↩</a></p></li>\n",
                    id,
                    process_inline(content),
                    id
                ));
            }
        }

        html.push_str("  </ol>\n");
        html.push_str("</div>\n");
        html
    }

    pub fn process_footnote_references(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Only process footnote references for footnotes that actually exist
        for id in &self.footnote_order {
            let pattern = format!("[^{}]", id);
            if result.contains(&pattern) {
                let footnote_ref = format!(
                    "<sup><a href=\"#fn-{}\" id=\"fnref-{}\" class=\"footnote-ref\">{}</a></sup>",
                    id, id, id
                );
                result = result.replace(&pattern, &footnote_ref);
            }
        }
        
        result
    }
}

pub fn is_footnote_definition(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with("[^") {
        if let Some(close_bracket) = trimmed.find("]:") {
            let id_part = &trimmed[2..close_bracket];
            return !id_part.is_empty() && id_part.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
        }
    }
    false
}

pub fn parse_footnote_definition(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.starts_with("[^") {
        if let Some(close_bracket) = trimmed.find("]:") {
            let id = trimmed[2..close_bracket].to_string();
            let content = trimmed[close_bracket + 2..].trim().to_string();
            if !id.is_empty() && !content.is_empty() {
                return Some((id, content));
            }
        }
    }
    None
}

