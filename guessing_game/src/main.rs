use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Print out a nice welcome message
    println!("\nWelcome, future Jake! You're playing \"Guess the Number!\" from the rust book");

    // Generate the secret number to be guessed, between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Infinite loop
    loop {
        println!("\nPlease input your guess:");

        // Create a new mutable string to hold the guess
        let mut guess = String::new();

        // Read in the next guess number, panic if it couldn't be read.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Set the new u32-type guess equal to the parsed value of the old string-type guess if
        // parsing succeeds, otherwise silently catch the error and continue.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print out the parsed guess
        println!("You guessed {guess}");

        // Compare the guess to the secret number and behave accordingly
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("DING DING DING!!");
                break;
            }
        }
    }
}
