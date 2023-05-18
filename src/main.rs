#![allow(dead_code, unused_variables)]
#![allow(clippy::upper_case_acronyms)]
//
//
//

mod cli;
mod errors;
mod template;

use crate::cli::{control_flow, get_cl_args};
use std::process::exit;

//
//
//

fn main() {
    handle_ctrlc();
    control_flow(get_cl_args()).expect("Encountered an IO error @ control_flow ... ");
    exit(0);
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
