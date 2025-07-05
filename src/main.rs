// markdown to html converter
// summer 30 june 2025

use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Debug)]
struct Config {
    input_file: String,
    theme: Option<String>,
    output_dir: String,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Self, String> {
        let mut input_file = None;
        let mut theme = None;
        let mut i = 1;

        while i < args.len() {
            match args[i].as_str() {
                "-t" => {
                    if i + 1 < args.len() {
                        theme = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err("-t requires a theme name".to_string());
                    }
                }
                arg if arg.starts_with('-') => {
                    return Err(format!("Unknown option '{}'", arg));
                }
                _ => {
                    if input_file.is_none() {
                        input_file = Some(args[i].clone());
                    } else {
                        return Err("Multiple input files specified".to_string());
                    }
                    i += 1;
                }
            }
        }

        match input_file {
            Some(path) => Ok(Config {
                input_file: path,
                theme,
                output_dir: "out".to_string(),
            }),
            None => Err("No input file specified".to_string()),
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} [-t <theme>] <input.md>", program_name);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -t <theme>    Select theme (light, dark, blue, green, purple, orange)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} example.md", program_name);
    eprintln!("  {} -t dark example.md", program_name);
    eprintln!("  {} -t blue example.md", program_name);
}

fn copy_css_directory(source: &str, dest: &str) -> Result<usize, Box<dyn std::error::Error>> {
    fs::create_dir_all(dest)?;
    
    let mut files_copied = 0;

    // main css files
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "css") {
            let file_name = path.file_name().unwrap();
            let dest_path = Path::new(dest).join(file_name);
            fs::copy(&path, dest_path)?;
            files_copied += 1;
        }
    }
    // themes
    let themes_source = Path::new(source).join("themes");
    if themes_source.exists() {
        let themes_dest = Path::new(dest).join("themes");
        fs::create_dir_all(&themes_dest)?;
        
        for entry in fs::read_dir(&themes_source)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "css") {
                let file_name = path.file_name().unwrap();
                let dest_path = themes_dest.join(file_name);
                fs::copy(&path, dest_path)?;
                files_copied += 1;
            }
        }
    }
    
    Ok(files_copied)
}

fn validate_theme(theme: &str, css_dir: &str) -> bool {
    let theme_file = Path::new(css_dir).join("themes").join(format!("{}.css", theme));
    theme_file.exists()
}

fn list_available_themes(css_dir: &str) -> Vec<String> {
    let themes_dir = Path::new(css_dir).join("themes");
    let mut themes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "css") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    themes.push(stem.to_string());
                }
            }
        }
    }
    
    themes.sort();
    themes
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading file: {}", config.input_file);
    
    if let Some(ref theme) = config.theme {
        if validate_theme(theme, "css") {
            println!("Using theme: {}", theme);
        } else {
            eprintln!("Warning: Theme '{}' not found. Available themes:", theme);
            for theme in list_available_themes("css") {
                eprintln!("  - {}", theme);
            }
            eprintln!("Falling back to default light theme.");
        }
    }
    

    fs::create_dir_all(&config.output_dir)?;
    
    // read and convert markdown
    let contents = fs::read_to_string(&config.input_file)?;
    let html = md_to_html::convert(&contents, config.theme.as_deref());
    
    // write HTML
    let output_html_path = Path::new(&config.output_dir).join("index.html");
    fs::write(&output_html_path, html)?;
    println!("Generated: {}", output_html_path.display());
    
    // copy CSS files
    if Path::new("css").exists() {
        let css_dest_dir = Path::new(&config.output_dir).join("css");
        match copy_css_directory("css", css_dest_dir.to_str().unwrap()) {
            Ok(count) => println!("Copied {} CSS files to {}/", count, css_dest_dir.display()),
            Err(e) => eprintln!("Warning: Could not copy CSS files: {}", e),
        }
    } else {
        println!("Warning: css/ directory not found");
    }
    
    println!("Output in '{}/' directory", config.output_dir);
    Ok(())
}

fn main() {
    println!("Markdown to HTML Converter");

    let args: Vec<String> = env::args().collect();
    
    let config = match Config::from_args(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            print_usage(&args[0]);
            process::exit(1);
        }
    };
    
    if let Err(err) = run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
