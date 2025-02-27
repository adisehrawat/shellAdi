use clap::{Arg, ArgMatches, Command};
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command as ProcessCommand};

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

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        let matches = Command::new("myshell")
            .subcommand(
                Command::new("exit")
                .about("Exit the shell")
                .arg(Arg::new("CODE").required(false))
            )
            .subcommand(
                Command::new("echo")
                    .about("Echoes input")
                    .arg(Arg::new("TEXT").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("type")
                    .about("Finds command type")
                    .arg(Arg::new("CMD").required(true)),
            )
            .try_get_matches_from(std::iter::once("myshell").chain(args.iter().copied()));

        match matches {
            Ok(matches) => handle_command(matches),
            Err(_) => external_handle(input),
        }
    }
}

fn handle_command(matches: ArgMatches) {
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
                let text: Vec<String> = values.map(|s| s.to_string()).collect();
                println!("{}", text.join(" "));
            }
        }
        Some(("type", sub_matches)) => {
            if let Some(cmd) = sub_matches.get_one::<String>("CMD") {
                match cmd.as_str() {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", cmd),
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
        _ => {}
    }
}

fn external_handle(input: &str) {
    let mut parts = input.split_whitespace();
    if let Some(cmd) = parts.next() {
        let args: Vec<&str> = parts.collect();
        if let Some(path) = find_executable_in_path(cmd) {
            match ProcessCommand::new(&path).args(&args).output() {
                Ok(output) => {
                    let program_name = Path::new(&path)
                        .file_name()
                        .unwrap_or_else(|| path.as_ref())
                        .to_string_lossy();
                    println!(
                        "Program was passed {} args (including program name).",
                        args.len() + 1
                    );
                    println!("Arg #0 (program name): {}", program_name);
                    for (i, arg) in args.iter().enumerate() {
                        println!("Arg #{}: {}", i + 1, arg);
                    }
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    for line in stdout_str.lines() {
                        if line.starts_with("Program Signature:") {
                            println!("{}", line.trim());
                            break;
                        }
                    }
                }
                Err(_) => println!("{}: command execution failed", cmd),
            }
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
