mod linux;
mod macos;
mod windows;

use std::env;
use std::env::consts::OS;
use std::process;

fn main() {
    // Get the current working directory.
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => {
            eprintln!("Error getting current working directory: {}", e);
            process::exit(1);
        }
    }

    // Detect the operating system and invoke the respective detection function.
    println!("Running on: {}", OS);
    match OS {
        "windows" => windows::detect(),
        "linux" => linux::detect(),
        "macos" => macos::detect(),
        _ => println!("Unknown operating system"),
    }
}
