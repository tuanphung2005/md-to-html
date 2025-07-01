// markdown to html converter
// summer 30 june 2025

use std::env;
use std::fs;
use std::path::Path;

use md_to_html::convert;

fn main() {
    println!("Markdown to HTML Converter");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input.md>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    
    // out directory
    let output_dir = "out";
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Error creating output directory: {}", e);
        std::process::exit(1);
    }

    println!("Reading file: {}", input_path);

    let contents = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_path, e);
            std::process::exit(1);
        }
    };

    let html = convert(&contents);

    // index.html
    let output_html_path = format!("{}/index.html", output_dir);
    match fs::write(&output_html_path, html) {
        Ok(_) => println!("Generated: {}", output_html_path),
        Err(e) => {
            eprintln!("Error writing HTML file: {}", e);
            std::process::exit(1);
        }
    }

    // css to /out
    let css_source_dir = "css";
    let css_dest_dir = format!("{}/css", output_dir);
    
    if Path::new(css_source_dir).exists() {

        if let Err(e) = fs::create_dir_all(&css_dest_dir) {
            eprintln!("Warning: Could not create CSS directory: {}", e);

        } else {

            if let Ok(entries) = fs::read_dir(css_source_dir) {

                let mut css_files_copied = 0;

                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("css") {
                            let file_name = path.file_name().unwrap();
                            let dest_path = format!("{}/{}", css_dest_dir, file_name.to_str().unwrap());
                            if let Ok(_) = fs::copy(&path, &dest_path) {
                                css_files_copied += 1;
                            }
                        }
                    }
                }
                if css_files_copied > 0 {
                    println!("Copied {} CSS files to {}/", css_files_copied, css_dest_dir);
                }
            }
        }
    } else {
        // uh oh
        println!("Warning: css/ directory not found");
    }

    println!("Output in '{}/' directory", output_dir);
}
