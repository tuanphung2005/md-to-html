// markdown to html converter
// summer 30 june 2025

use std::env;
use std::fs;

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &trimmed[2..]));
        } else if trimmed.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &trimmed[3..]));
        } else if trimmed.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &trimmed[4..]));
        } else {
            html.push_str(&format!("<p>{}</p>\n", trimmed));
        }
    }

    html
}

fn main() {
    println!("Markdown to HTML Converter");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <input.md> [output.html]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = 
        if args.len() == 3 {
            args[2].clone()
        } else {
            input_path.replace(".md", ".html")
        };

    println!("file {}", input_path);

    let contents = fs::read_to_string(input_path).
        expect("Something went wrong reading the file");

    // println!("{}", contents);

    let html = markdown_to_html(&contents);
    println!("{html}");

    match fs::write(&output_path, html) {
        Ok(_) => println!("Successfully wrote to {}", output_path),
        Err(e) => eprintln!("Error writing to {}: {}", output_path, e),
    }
}
