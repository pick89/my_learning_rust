use std::cmp::Ordering;
use std::io;
use rand::Rng;
use colored::*;

const MAX_ATTEMPTS: u32 = 7; // Maximum number of attempts allowed
// This is a simple guessing game where the player has to guess a number between 1 and 100.
fn main() {
    loop {
        run_game();
        // Ask the player if they want to play again after the game ends
        if !ask_restart() {
            println!("👋 Thanks for playing the guessing game. Goodbye!");
            break;
        }
    }
}
// Function to run the guessing game
fn run_game() {
    println!("🎯 Welcome to the guessing game!\n");

    let secret_number = rand::rng().random_range(1..=100);
    println!("🤔 I have selected a secret number between 1 and 100. You have {} attempts!", MAX_ATTEMPTS);
    let mut previous_guess = Vec::new(); // Vector to store previous guesses
    for attempt in 1..=MAX_ATTEMPTS {
        print_attempt_prompt(attempt);
        
        let guess = match read_user_input() {
            Some(num) => num,
            None => {
                println!("⚠️ Invalid input. Please enter a valid number.");
                continue;
            }
        };
        previous_guess.push(guess);
        // Check if the guess is within the valid range
        println!("🔢 You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small!"),
            Ordering::Greater => println!("📈 Too big!"),
            Ordering::Equal => {
                println!("{}",format!("🎉 Correct! The number was: {} in {} attempts!",secret_number,attempt).green().bold());
                return;
            }
        }
        println!("{}", format!("Previous guesses: {:?}", previous_guess).red().bold());
    }
    // If the player runs out of attempts, reveal the secret number
    println!("💥 Game over! The number was: {}.", secret_number);
}
// Function to read user input and return it as an Option<u32>
fn read_user_input() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
    
}
// Function to print the prompt for the current attempt
fn print_attempt_prompt(attempt: u32) {
    if attempt == MAX_ATTEMPTS {
        println!("\n⚠️ Last Attempt ({}): Enter your guess (1–100):", attempt);
    } else {
        println!("\n🔁 Attempt {}: Enter your guess (1–100):", attempt);
    }
}
// Function to ask the player if they want to restart the game
fn ask_restart() -> bool {
    println!("\n💡 Would you like to play again? (yes/no)");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    // Convert input to lowercase and trim whitespace
    let response = response.trim().to_lowercase();

    response == "yes"
}  