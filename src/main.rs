mod commands;
mod executor;
mod utils;
mod cli;

use std::io::{self, Write};

fn main() {

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!("$ ");
        stdout.flush().unwrap();
        input.clear();

        if stdin.read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        

        match cli::parse_command(input) {
            Ok(matches) => commands::handle_command(matches),
            Err(_) => executor::external_handle(input),
        }
    }
}




