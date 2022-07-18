use std::io::stdin;

fn main() {
    println!("Hello, world!");

    // Create string to hold user input
    let mut user_input = String::new();

    // Get user input
    stdin()
        .read_line(&mut user_input)
        .expect("Testing str input");

    // Test?
    test(user_input);
}

fn test(input: String) {
    println!("PRINTING: {}", input);
}
