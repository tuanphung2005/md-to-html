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

    pub fn to_html(&self) -> String {
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
        
        html.push_str(&self.content);
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        html
    }
}
