use std::cmp::Ordering;
use std::io;
use rand::Rng;
use rand::rng;

fn main() {
    println!("🎯 Welcome to the guessing game!\n");
    let secret_number = rng().random_range(1..=100); // Generate a random number between 1 and 100
    println!("🤔 I have selected a secret number between 1 to 100. Can you guess it?");
    let mut attempts = 0; // Initialize attempts to 0
    loop {
        if attempts == 0{
            println!("Enter your guess (1–100):");
        }else{
            println!("Attempt {}: Enter your guess (1–100):", attempts + 1);
        }

        let guess = match read_user_input() {
            Some(num) => num,
            None => {
                println!("\n⚠️ Invalid input. Please enter a valid number.");
                continue;
            }
        };
    attempts += 1;

        println!("🔢 You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!"),
            Ordering::Greater => println!("\nToo big!"),
            Ordering::Equal => {
            println!("🎉 Correct! The number was: {}\n\n🧮 You guessed it in {} attempts!", secret_number, attempts);
                break;
            }
        }
    }
}

fn read_user_input() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}
