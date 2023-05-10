#![allow(dead_code, unused_variables)]
//
//
//

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::stdin;
use std::io::LineWriter;
use std::process::exit;
//

struct Badge<'a>(&'a str);

enum LicenseBadge {
    MIT,
    Apache,
    Mozilla,
    GNU,
}

impl<'a> Badge<'a> {
    fn write(kind: LicenseBadge, writer: &mut LineWriter<File>) -> Self {
        match kind {
            LicenseBadge::MIT => Badge("[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)"),
            LicenseBadge::Apache => Badge("[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)"),
            LicenseBadge::Mozilla => Badge("[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)"),
            LicenseBadge::GNU => Badge("[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)")
        }
    }
}

//

#[derive(Debug)]
struct WriteError {
    message: String,
}

fn main() {
    control_flow().expect("Encountered an error @ control_flow ... ");
}

fn control_flow() -> std::io::Result<()> {
    let mut line_writer = LineWriter::new(
        create_file("README.md").expect("Should have initialized file upon launch."),
    );

    // Get project name
    prompt_for("name");
    let project_name = to_formatted(&get_input(), "name");
    line_writer.write(project_name.as_bytes())?;
    spacer(&mut line_writer);

    // Get subtitle
    match ask_for("subtitle") {
        true => {
            prompt_for("subtitle");
            let subtitle = to_formatted(&get_input(), "subtitle");
            line_writer.write(subtitle.as_bytes())?;
        }
        false => println!("Subtitle will not be included."),
    };

    spacers(&mut line_writer);

    println!("What license will the project be under?\nOptions: MIT | Apache | Mozilla | GNU");
    let license_type = get_input();
    println!("License Type: {}", license_type);

    exit(0);
}

fn get_input() -> String {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Stdin should have read user input into string buffer.");

    buffer
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
        "subtitle" => format!("### {s}"),
        _ => "to_formatted received an invalid argument ... ".to_string(),
    }
}

fn prompt_for(opt: &str) {
    match opt {
        "name" => println!("Enter the name of your project:"),
        "subtitle" => println!("Enter your subtitle:"),
        "license" => {
            println!("Choose a license for your project.\nOptions: MIT | Apache | Mozilla | GNU")
        }
        _ => println!("Reached wildcard match arm"),
    }
}

fn ask_for(opt: &str) -> bool {
    let prompt = || println!("Include a {}?", opt);
    match get_input().trim() {
        "Y" | "y" | "yes" | "YES" => true,
        "N" | "n" | "no" | "NO" => false,
        _ => false,
    }
}

fn spacer<'a>(writer: &mut LineWriter<File>) {
    writer
        .write("___\n".as_bytes())
        .expect("Writer should have written spacer characters");
}

fn spacers(writer: &mut LineWriter<File>) {
    spacer(writer);
    spacer(writer);
    spacer(writer);
}
