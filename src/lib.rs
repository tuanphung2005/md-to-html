pub mod parser;
pub mod html;

pub fn convert(markdown: &str, theme: Option<&str>) -> String {
    parser::markdown_to_html(markdown, theme)
}