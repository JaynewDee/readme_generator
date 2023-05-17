#![allow(dead_code, unused_variables)]
#![allow(clippy::upper_case_acronyms)]
//
//
//

mod template;

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{stdin, stdout};
use std::process::exit;
use template::{ Badge, LicenseBadge};

//
//
//
//

#[derive(Debug)]
struct WriteError {
    message: String,
}

fn main() {
    handle_ctrlc(); 
    control_flow(get_cl_args()).expect("Encountered an IO error @ control_flow ... ");
    exit(0);
}

fn control_flow(cl_args: Vec<String>) -> std::io::Result<()> {
    let mut line_writer = LineWriter::new(
        create_file("README.md").expect("Should have initialized file upon launch."),
    );

    // If no arguments passed,
    // write default template

    if cl_args.len() < 2 {
        let badge = Badge::generate(LicenseBadge::Default).0;
        let template = template::default(&badge);
        line_writer.write_all(template.as_bytes())?;
        println!("Default README template written to current working directory.");
        exit(0)
    }

    // Get project name
    prompt_for("name");
    let project_name = to_formatted(&get_input(), "name");
    line_writer.write_all(project_name.as_bytes())?;

    // Get subtitle
    match ask_for("subtitle") {
        true => {
            prompt_for("subtitle");
            let subtitle = to_formatted(&get_input(), "subtitle");
            line_writer.write_all(subtitle.as_bytes())?;
        }
        false => println!("Subtitle will not be included."),
    };

    // Get license
    prompt_for("license");
    let license_type = get_input();
    let kind = Badge::match_str(&license_type);
    let license = Badge::generate(kind);
    line_writer.write_all(license.0.as_bytes())?;

    // Write table of contents
    let byte_string = template::toc().as_bytes();
    line_writer.write_all(byte_string)?;

    Ok(())
}

fn get_input() -> String {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Stdin should have read user input into string buffer.");

    buffer
}

fn create_file(name: &str) -> Result<File, WriteError> {
    match File::create(name) {
        Ok(file) => Ok(file),
        Err(_) => Err(WriteError {
            message: "Failed to initialize readme file ... ".to_string(),
        }),
    }
}

fn to_formatted(s: &str, section: &str) -> String {
    match section {
        "name" => format!("# {s}\n"),
        "subtitle" => format!("### {s}\n"),
        _ => "to_formatted received an invalid argument ... ".to_string(),
    }
}

fn flush_out() {
    stdout()
        .flush()
        .expect("Should have flushed stdout stream ... ");
}

fn prompt_for(opt: &str) {
    match opt {
        "name" => {
            println!("Enter the name of your project:");
            flush_out();
        }
        "subtitle" => {
            println!("Enter your subtitle:");
            flush_out();
        }
        "license" => {
            println!("Choose a license for your project.\nOptions: MIT | Apache | Mozilla | GNU");
            flush_out();
        }
        _ => {
            println!("Reached wildcard match arm");
            flush_out();
        }
    }
}

fn ask_for(opt: &str) -> bool {
    println!("Include a {}?", opt);
    match get_input().trim() {
        "Y" | "y" | "yes" | "YES" => true,
        "N" | "n" | "no" | "NO" => false,
        _ => false,
    }
}

fn get_cl_args() -> Vec<String> {
    std::env::args().collect()
}

fn handle_ctrlc() {
    ctrlc::set_handler(|| {
        println!("Received EXIT event.");
        println!("Shutting down gracefully ... ");
        exit(0);
    })
    .expect("Error setting handler for Ctrl-C ...");
}

/////////////////////
/////////////////////
/////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_formatted() {
        let input_str = "Super Cool Project";
        let section = "name";
        let result = to_formatted(input_str, section);
        let correct = "# Super Cool Project\n".to_string();
        assert!(result == correct);

        let input_str = "Very cool and blazingly fast";
        let section = "subtitle";
        let result = to_formatted(input_str, section);
        let correct = "### Very cool and blazingly fast\n".to_string();
        assert!(result == correct);
    } 
}

/////////////////////////
