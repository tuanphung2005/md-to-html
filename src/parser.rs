pub fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            // Handle empty lines better
            continue;
        } else if trimmed.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &trimmed[2..]));
        } else if trimmed.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &trimmed[3..]));
        } else if trimmed.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &trimmed[4..]));
        } else if trimmed.starts_with("#### ") {
            html.push_str(&format!("<h4>{}</h4>\n", &trimmed[5..]));
        } else if trimmed.starts_with("##### ") {
            html.push_str(&format!("<h5>{}</h5>\n", &trimmed[6..]));
        } else if trimmed.starts_with("###### ") {
            html.push_str(&format!("<h6>{}</h6>\n", &trimmed[7..]));
        } else {
            html.push_str(&format!("<p>{}</p>\n", trimmed));
        }
    }

    html
}