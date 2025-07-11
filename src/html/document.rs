use crate::parser::toc::TableOfContents;

pub struct HtmlDocument {
    content: String,
}

impl HtmlDocument {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn add_content(&mut self, html: &str) {
        self.content.push_str(html);
    }

    pub fn to_html(&self, toc: &TableOfContents, theme: Option<&str>) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
        html.push_str("<title>Markdown to HTML</title>\n");
        html.push_str("<link rel=\"stylesheet\" href=\"css/main.css\">\n");
        
        // theme if specified
        if let Some(theme_name) = theme {
            html.push_str(&format!("<link rel=\"stylesheet\" href=\"css/themes/{}.css\">\n", theme_name));
        }

        
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        
        
        // toc as a sidebar
        let toc_html = toc.generate_html();
        if !toc_html.is_empty() {
            html.push_str(&toc_html);
        }
        
        // main content wrapper
        html.push_str("<div class=\"toc-content\">\n");
        html.push_str("<div class=\"content\">\n");
        html.push_str(&self.content);
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        html
    }
}
