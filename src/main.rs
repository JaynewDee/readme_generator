//
//
//

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::stdin;
use std::io::LineWriter;

//
#[derive(Debug)]
struct WriteError {
    message: String,
}

fn main() {
    create_file("README.md").expect("Should have initialized file upon launch.");
    println!("What is the name of your project?\n");
    let project_name = get_input();
    println!("Project Name: {}", to_formatted(&project_name, "name"));

    println!("What license will the project have?\n Options: MIT");
    let license_type = get_input();
    println!("License Type: {}", license_type);
}

fn get_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Stdin should have read user input into string buffer.");
    input
}

fn create_file(name: &str) -> Result<File, WriteError> {
    let write_result = File::create(&name);
    match write_result {
        Ok(file) => Ok(file),
        Err(_) => Err(WriteError {
            message: "Failed to initialize readme file ... ".to_string(),
        }),
    }
}

fn to_formatted(s: &str, section: &str) -> String {
    match section {
        "name" => format!("# {s}"),
        _ => format!("placeholder"),
    }
}

fn make_line_writer(file: File) -> LineWriter<File> {
    LineWriter::new(file)
}
