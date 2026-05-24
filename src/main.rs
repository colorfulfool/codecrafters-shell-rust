use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut statement = String::new();
        io::stdin().read_line(&mut statement).unwrap();

        let mut parts = statement.splitn(2, " ");

        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("");

        match command {
            "exit" => break,
            "echo" => println!("{}", args),
            cmd => println!("{}: command not found", cmd),
        }
    }
}
