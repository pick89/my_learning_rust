use std::cmp::Ordering; 
use std::io;

use rand::Rng;

fn main() {
    // Launch the game
    guessing_game();
}

fn guessing_game() {
    println!("Hello and welcome to the guessing game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    // Launch the loop
    loop {
        println!("Please input your guess (between 1 and 100):");

        // Create a mutable string to store input
        let mut guess = String::new();

        // Read the input value
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the input to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number: {}", secret_number);
                break;
            }
        }
    }
}
