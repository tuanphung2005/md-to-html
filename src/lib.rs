pub mod parser;
pub mod html;

pub use parser::markdown_to_html;

pub fn convert(markdown: &str) -> String {
    markdown_to_html(markdown)
}