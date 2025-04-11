use std::{io::{stdin, stdout, Write}, process::Command};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        // Create a mutable string to store user input
        let mut input = String::new();

        // Read a line from standard input (keyboard) into the input string
        stdin().read_line(&mut input).unwrap();

        if input.is_empty() {
            continue
        }
    
        // Remove trailing whitespace (like newlines) from the input
        let mut parts = input.trim().split_whitespace();

        let command = parts.next().unwrap();

        let args = parts;
    
        // Execute the command entered by the user
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        let _ = child.wait();
    }
}
