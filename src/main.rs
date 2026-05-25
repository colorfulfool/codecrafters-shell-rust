use std::env;
use std::io::Write;
use std::io::{self, ErrorKind};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::process::Command;

fn print_type(command: &str) {
    match command {
        "exit" | "echo" | "type" => println!("{} is a shell builtin", command),
        _ => match find_executable(command) {
            Ok(executable) => println!("{} is {}", command, executable.to_string_lossy()),
            Err(_error) => println!("{}: not found", command),
        },
    }
}

fn find_executable(command: &str) -> Result<PathBuf, io::Error> {
    for directory in env::split_paths(&env::var("PATH").unwrap_or_default()) {
        let filename = directory.join(command);
        match filename.metadata() {
            Ok(meta) => {
                if meta.is_file() && meta.mode() & 0o100 != 0 {
                    return Ok(filename);
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    return Err(io::Error::new(
        ErrorKind::NotFound,
        format!("{}: not found", command),
    ));
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut statement = String::new();
        io::stdin().read_line(&mut statement).unwrap();

        let mut parts = statement.splitn(2, " ");

        let command = parts.next().unwrap_or("").trim();
        let args = parts.next().unwrap_or("").trim();

        match command {
            "exit" => break,
            "echo" => println!("{}", args),
            "type" => print_type(args),
            cmd => match find_executable(cmd) {
                Ok(_filename) => match Command::new(format!("{} {}", cmd, args)).output() {
                    Ok(output) => println!("Output: {}", String::from_utf8_lossy(&output.stdout)),
                    Err(e) => println!("Failed to execute: {}", e),
                },
                Err(error) => println!("{}", error),
            },
        }
    }
}
