// markdown to html converter
// summer 30/june/2025

use std::env;
use std::fs;

fn main() {
    println!("Markdown to HTML Converter");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("{}", contents);
}
