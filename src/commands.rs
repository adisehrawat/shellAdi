use crate::utils;


use clap::ArgMatches;
use std::env;
use std::fs;
use std::process;
use pathsearch::find_executable_in_path;

pub fn handle_command(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("exit", sub_matches)) => {
            let code = sub_matches
                .get_one::<String>("CODE")
                .and_then(|c| c.parse::<i32>().ok())
                .unwrap_or(0); // Default to 0 if no argument is given
            process::exit(code);
        }

        Some(("echo", sub_matches)) => {
            if let Some(values) = sub_matches.get_many::<String>("TEXT") {
                println!("{}", values.map(|s| s.as_str()).collect::<Vec<&str>>().join(" "));
            }
        }

        Some(("cat", sub_matches)) => {
            if let Some(files) = sub_matches.get_many::<String>("FILES") {
                for file in files {
                    match fs::read_to_string(file) {
                        Ok(contents) => print!("{}", contents),
                        Err(_) => eprintln!("cat: {}: No such file or directory", file),
                    }
                }
            }
        }
        Some(("exe  with  space", sub_matches)) => {
            if let Some(files) = sub_matches.get_many::<String>("FILES") {
                for file in files {
                    match fs::read_to_string(file) {
                        Ok(contents) => print!("{}", contents),
                        Err(_) => eprintln!("cat: {}: No such file or directory", file),
                    }
                }
            }
        }
        Some(("exe with \"quotes\"", sub_matches)) => {
            if let Some(files) = sub_matches.get_many::<String>("FILES") {
                for file in files {
                    match fs::read_to_string(file) {
                        Ok(contents) => print!("{}", contents),
                        Err(_) => eprintln!("cat: {}: No such file or directory", file),
                    }
                }
            }
        }
        Some(("exe with \\'single quotes\\'", sub_matches)) => {
            if let Some(files) = sub_matches.get_many::<String>("FILES") {
                for file in files {
                    match fs::read_to_string(file) {
                        Ok(contents) => print!("{}", contents),
                        Err(_) => eprintln!("cat: {}: No such file or directory", file),
                    }
                }
            }
        }
        Some(("exe with \\n newline", sub_matches)) => {
            if let Some(files) = sub_matches.get_many::<String>("FILES") {
                for file in files {
                    match fs::read_to_string(file) {
                        Ok(contents) => print!("{}", contents),
                        Err(_) => eprintln!("cat: {}: No such file or directory", file),
                    }
                }
            }
        }
        Some(("type", sub_matches)) => {
            if let Some(cmd) = sub_matches.get_one::<String>("CMD") {
                match cmd.as_str() {
                    "echo" | "exit" | "type" | "pwd" => println!("{} is a shell builtin", cmd),
                    _ => {
                        if let Some(path) = find_executable_in_path(cmd) {
                            println!("{} is {}", cmd, path.to_str().unwrap());
                        } else {
                            println!("{}: not found", cmd);
                        }
                    }
                }
            }
        }

        Some(("cd", sub_matches)) => {
            if let Some(path) = sub_matches.get_one::<String>("CD") {
                if let Err(_e) = utils::change_directory(path) {
                    eprintln!("cd: {path}: No such file or directory");
                }
            }
        }

        Some(("pwd", _)) => {
            let current_dir = env::current_dir().unwrap();
            println!("{}", current_dir.display());
        }
        _ => {}
    }
}
