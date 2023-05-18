use crate::errors::{ControlError, WriteError};
use crate::template::{default, prompted, Badge, LicenseBadge, PromptOptions};
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{stdin, stdout};
use std::process::exit;
//
//
//

pub fn get_cl_args() -> Vec<String> {
    std::env::args().collect()
}

pub fn init_write_file() -> Result<LineWriter<File>, WriteError> {
    let line_writer = LineWriter::new(create_file("README.md")?);
    Ok(line_writer)
}

fn create_file(name: &str) -> Result<File, WriteError> {
    let file = File::create(name)?;
    Ok(file)
}

pub fn control_flow(cl_args: Vec<String>) -> Result<String, ControlError> {
    let mut line_writer = init_write_file().unwrap();

    // If no arguments passed,
    // write default template
    if cl_args.len() < 2 {
        from_default(&mut line_writer)?;
    };

    // if first arg is `prompt`,
    // initiate prompt flow
    if cl_args[1].trim().to_lowercase() == String::from("prompt") {
        from_prompt_flow(&mut line_writer)?
    }

    Ok("EXIT".to_string())
}

fn from_prompt_flow(writer: &mut LineWriter<File>) -> std::io::Result<()> {
    prompt_for("title");
    let user_input = &get_input().expect("Expected title input!");
    let title = to_formatted(&user_input, "title");

    let st = "subtitle";
    prompt_for(st);
    let user_input = &get_input().expect("Expected subtitle input!");
    let subtitle = to_formatted(&user_input, st);

    prompt_for("license");
    let license_type = &get_input().expect("Expected license to be a Some value");
    let kind = Badge::match_str(&license_type);
    let license_badge = Badge::generate(kind);

    let mut sections: Vec<String> = vec![];

    prompt_for("section");
    while let Some(sect) = get_input() {
        sections.push(sect.trim().to_string());
        flush_out();
    }

    let options = PromptOptions {
        title: &title,
        subtitle: &subtitle,
        badge: license_badge,
        sections,
    };

    let template = prompted(options);
    writer.write_all(template.as_bytes())?;
    exit(0)
}

fn from_default(writer: &mut LineWriter<File>) -> std::io::Result<()> {
    let badge = Badge::generate(LicenseBadge::Default).0;
    let template = default(&badge);

    writer.write_all(template.as_bytes())?;

    println!("\nDefault README template written to current working directory.");
    exit(0)
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Stdin should have read user input into string buffer.");

    if buffer.trim().to_lowercase() == "d" {
        return None;
    } else {
        flush_out();
        return Some(buffer);
    }
}
// TODO: Remove/handle later
fn to_formatted(s: &str, header: &str) -> String {
    match header {
        "title" | "subtitle" => format!("{s}\n"),
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
        "title" => {
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
        "section" => {
            println!("Add sections: (type `d` then ENTER when finished)");
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

    match get_input().expect("Ask_for prompt failed ... ").trim() {
        "Y" | "y" | "yes" | "YES" => true,
        "N" | "n" | "no" | "NO" => false,
        _ => false,
    }
}
