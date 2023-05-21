//
//
//
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

enum CountError {
    Negative,
    BeyondMax,
    Parse
}

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
    if cl_args[1].trim().to_lowercase() == *"prompt" {
        from_prompt_flow(&mut line_writer)?
    }

    Ok("EXIT".to_string())
}

fn from_prompt_flow(writer: &mut LineWriter<File>) -> std::io::Result<()> {
    prompt_for("title");
    let title = &get_input().expect("Expected title input!");
    

    prompt_for("subtitle");
    let subtitle = &get_input().expect("Expected subtitle input!");

    prompt_for("images");
    let user_input = &get_input().expect("Expected number of images.");
    // If count cannot be parsed, default to 0
    let image_count = user_input.trim().parse::<u8>().unwrap_or(0);


    prompt_for("license");
    let license_type = &get_input().expect("Expected license to be a Some value");
    let kind = Badge::match_str(license_type);
    let license_badge = Badge::generate(kind);

    let mut sections: Vec<String> = vec![];

    prompt_for("section");
    while let Some(sect) = get_input() {
        sections.push(sect.trim().to_string());
        flush_out();
    }

    let options = PromptOptions {
        title: &title,
        subtitle: subtitle.trim(),
        badge: license_badge,
        sections,
        image_count
    };

    let template = prompted(options);
    writer.write_all(template.as_bytes())?;
    println!("SUCCESS");
    exit(0)
}

fn from_default(writer: &mut LineWriter<File>) -> std::io::Result<()> {
    let badge = Badge::generate(LicenseBadge::Default).0;
    let template = default(badge);

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
        None
    } else {
        flush_out();
        Some(buffer)
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
        "images" => {
            println!("Number of image placeholders:");
            flush_out()
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

