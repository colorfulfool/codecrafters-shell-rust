use std::io::{self, Write};

fn print_type(command: &str) {
    match command {
        "exit" | "echo" | "type" => println!("{} is a shell builtin", command),
        _ => println!("{}: not found", command),
    }
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
            cmd => println!("{}: command not found", cmd),
        }
    }
}
