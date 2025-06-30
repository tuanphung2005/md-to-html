
use super::blocks::Header;

pub struct TocEntry {
    pub id: String,
    pub text: String,
    pub level: u8,
}

pub struct TableOfContents {
    entries: Vec<TocEntry>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_header(&mut self, header: &Header) {
        self.entries.push(TocEntry {
            id: header.id.clone(),
            text: header.text.clone(),
            level: header.level,
        });
    }

    pub fn generate_html(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }

        let mut html = String::new();
        html.push_str("<div class=\"table-of-contents\">\n");
        html.push_str("  <h2>Table of Contents</h2>\n");
        html.push_str("  <ul class=\"toc-list\">\n");

        for entry in &self.entries {
            html.push_str(&format!(
                "    <li class=\"toc-level-{}\"><a href=\"#{}\">{}</a></li>\n",
                entry.level, entry.id, entry.text
            ));
        }

        html.push_str("  </ul>\n");
        html.push_str("</div>\n");
        html
    }
}
