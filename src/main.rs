#![allow(dead_code, unused_variables)]
#![allow(clippy::upper_case_acronyms)]
//
//
//

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{stdin, stdout};
use std::process::exit;

//
//
//
#[derive(Debug)]
struct Badge<'a>(&'a str);

enum LicenseBadge {
    MIT,
    Apache,
    Mozilla,
    GNU,
    Default,
}

impl<'a> Badge<'a> {
    fn match_str(kind: &str) -> LicenseBadge {
        match kind.trim() {
            "MIT" => LicenseBadge::MIT,
            "Apache" => LicenseBadge::Apache,
            "Mozilla" => LicenseBadge::Mozilla,
            "GNU" => LicenseBadge::GNU,
            _ => LicenseBadge::Default,
        }
    }
    fn generate(kind: LicenseBadge) -> Self {
        match kind {
            LicenseBadge::MIT => Badge("[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)"),
            LicenseBadge::Apache => Badge("[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)"),
            LicenseBadge::Mozilla => Badge("[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)"),
            LicenseBadge::GNU => Badge("[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)"),
            LicenseBadge::Default => Badge("[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)")
        }
    }
}

//
fn toc() -> &'static str {
r"
## Description  
<Describe the Application>

## Table of Contents  

*::: [Installation](#installation) :::*

*::: [Usage](#usage) :::*

*::: [Contributing](#contributing) :::*

*::: [Tests](#test) :::*

*::: [Questions](#questions) :::*

## Installation  
<Instructions go here>  

## Usage  
<How to use app/repo>

## Contributing
<Special mentions / how to contribute>

## Tests  
<How to run tests / performance explanation>

## Questions  
<How to get in contact>
"
}

#[derive(Debug)]
struct WriteError {
    message: String,
}

fn main() {
    control_flow().expect("Encountered an IO error @ control_flow ... ");
    exit(0);
}

fn control_flow() -> std::io::Result<()> {
    let mut line_writer = LineWriter::new(
        create_file("README.md").expect("Should have initialized file upon launch."),
    );

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
    let byte_string = toc().as_bytes();
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

fn spacer(writer: &mut LineWriter<File>) {
    writer
        .write_all("___\n".as_bytes())
        .expect("Writer should have written spacer characters");
}

fn spacers(writer: &mut LineWriter<File>) {
    spacer(writer);
    spacer(writer);
    spacer(writer);
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
        let correct = "# Super Cool Project".to_string();
        assert!(result == correct);

        let input_str = "Very cool and blazingly fast";
        let section = "subtitle";
        let result = to_formatted(input_str, section);
        let correct = "### Very cool and blazingly fast".to_string();
        assert!(result == correct);
    }
}

/////////////////////////
