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

fn main() -> std::io::Result<()> {
    let mut line_writer = LineWriter::new(
        create_file("README.md").expect("Should have initialized file upon launch."),
    );

    // Get project name
    println!("Enter the name of your project:");
    let project_name = to_formatted(&get_input(), "name");

    line_writer.write(project_name.as_bytes())?;
    line_writer.write(spacer())?;
    line_writer.write(spacer())?;
    line_writer.write(spacer())?;
    //
    //
    let wants_subtitle = ask_for("subtitle");


    // Get license options
    //    println!("What license will the project have?\n Options: MIT");
    //   let license_type = get_input();
    //  println!("License Type: {}", license_type);
    // Ok(())
    Ok(())
}

//

fn get_input() -> String {
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Stdin should have read user input into string buffer.");
    buffer 
}

//

fn create_file(name: &str) -> Result<File, WriteError> {
    let write_result = File::create(&name);
    match write_result {
        Ok(file) => Ok(file),
        Err(_) => Err(WriteError {
            message: "Failed to initialize readme file ... ".to_string(),
        }),
    }
}

//

fn to_formatted(s: &str, section: &str) -> String {
    match section {
        "name" => format!("# {s}"),
        _ => "to_formatted received an invalid argument ... ".to_string(),
    }
}

//

fn ask_for(opt: &str) -> bool {
    let prompt = || println!("Include a {}?", opt);
    
    let answer = match opt {
        "subtitle" => {
            prompt();
            get_input()
        },
        _ => String::new() 
    };
    match &answer[..] {
        "Y" | "y" | "yes" | "YES" => true,
        "N" | "n" | "no" | "NO" => false,
        _ => false
    }
}

//

fn spacer<'a>() -> &'a [u8] {
    "___\n".as_bytes()
}
