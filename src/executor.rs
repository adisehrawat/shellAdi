use std::path::Path;
use std::process::Command as ProcessCommand;
use pathsearch::find_executable_in_path;

pub fn external_handle(input: &str) {
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
