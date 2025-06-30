use super::inline::process_inline;

pub fn process_header(line: &str) -> Option<String> {
    if line.starts_with("###### ") {
        Some(format!("<h6>{}</h6>\n", process_inline(&line[7..])))
    } else if line.starts_with("##### ") {
        Some(format!("<h5>{}</h5>\n", process_inline(&line[6..])))
    } else if line.starts_with("#### ") {
        Some(format!("<h4>{}</h4>\n", process_inline(&line[5..])))
    } else if line.starts_with("### ") {
        Some(format!("<h3>{}</h3>\n", process_inline(&line[4..])))
    } else if line.starts_with("## ") {
        Some(format!("<h2>{}</h2>\n", process_inline(&line[3..])))
    } else if line.starts_with("# ") {
        Some(format!("<h1>{}</h1>\n", process_inline(&line[2..])))
    } else {
        None
    }
}

pub fn process_paragraph(line: &str) -> String {
    format!("<p>{}</p>\n", process_inline(line))
}
