use std::io::stdin;

fn main() {
    println!("Hello, world!");

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Testing str input");

    test(user_input);
}

fn test(input: String) {
    println!("PRINTING: {}", input);
}
