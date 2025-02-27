use std::env;
use std::io;
use std::path::Path;
use dirs;

pub fn change_directory(path: &str) -> io::Result<()> {
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
