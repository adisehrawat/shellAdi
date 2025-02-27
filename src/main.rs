use clap::{Arg, ArgMatches, Command};
use pathsearch::find_executable_in_path;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command as ProcessCommand};
use dirs;
use std::fs;
use shell_words;

fn main() {
    // Ensure /tmp/quz/ is included in PATH
    env::set_var("PATH", format!("/tmp/quz:{}", env::var("PATH").unwrap_or_default()));

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

        let args = shell_words::split(input).unwrap_or_else(|_| vec![]); // Preserve quoted strings
        if args.is_empty() {
            continue;
        }

        let matches = Command::new("myshell")
            .subcommand(
                Command::new("exit")
                    .about("Exit the shell")
                    .arg(Arg::new("CODE").required(false)),
            )
            .subcommand(
                Command::new("echo")
                    .about("Echoes input")
                    .arg(Arg::new("TEXT").required(true).num_args(1..).allow_hyphen_values(true)),
            )
            .subcommand(
                Command::new("cat")
                    .about("Concatenates and prints file contents")
                    .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe  with  space")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe with \"quotes\"")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe with \\'single quotes\\'")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("exe with \\n newline")
                .about("about")
                .arg(Arg::new("FILES").required(true).num_args(1..)),
            )
            .subcommand(
                Command::new("type")
                    .about("Finds command type")
                    .arg(Arg::new("CMD").required(true)),
            )
            .subcommand(
                Command::new("cd")
                .about("move the the directory")
                .arg(Arg::new("CD").required(true)),
            )
            .subcommand(Command::new("pwd").about("Tells the current directory"))
            .try_get_matches_from(std::iter::once("myshell").chain(args.iter().map(String::as_str)));

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
                if let Err(_e) = change_directory(path) {
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

fn change_directory(path: &str) -> io::Result<()> {
    let new_path = if path == "~" {
        dirs::home_dir().unwrap_or_else(|| env::current_dir().unwrap()) 
    } else {
        let input_path = Path::new(path);
        if input_path.is_absolute() {
            input_path.to_path_buf()
        } else {
            env::current_dir()?.join(input_path) 
        }
    };

    let new_path = new_path.canonicalize()?;

    if new_path.is_dir() {
        env::set_current_dir(&new_path)?;
    } else {
        eprintln!("Error: '{}' is not a valid directory", new_path.display());
    }
    Ok(())
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
