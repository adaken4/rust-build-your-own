use std::{io::stdin, process::Command};

fn main() {
    // Create a mutable string to store user input
    let mut input = String::new();

    // Read a line from standard input (keyboard) into the input string
    stdin().read_line(&mut input).unwrap();

    // Remove trailing whitespace (like newlines) from the input
    let command = input.trim();

    // Execute the command entered by the user
    Command::new(command)
        .spawn()
        .unwrap();
}
