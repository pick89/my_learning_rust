use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    println!("🎯 Welcome to the guessing game!\n");

    let secret_number = rand::rng().random_range(1..=100);
    println!("🤔 I have selected a secret number between 1 and 100. You have 3 attempts!");

    let mut attempts = 0;

    loop {
        attempts += 1;

        if attempts == 1 {
            println!("🔁 Attempt 1: Enter your guess (1–100):");
        } else if attempts == 3 {
            println!("\n⚠️ Last Attempt (3): Enter your guess (1–100):");
        } else {
            println!("\n🔁 Attempt {}: Enter your guess (1–100):", attempts);
        }

        let guess = match read_user_input() {
            Some(num) => num,
            None => {
                println!("\n⚠️ Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("🔢 You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small!"),
            Ordering::Greater => println!("📈 Too big!"),
            Ordering::Equal => {
                println!("\n🎉 Correct! The number was: {}\n🧮 You guessed it in {} attempts!", secret_number, attempts);
                break;
            }
        }

        if attempts >= 7 {
            println!("\n💥 Game over! You've used all 3 attempts. The number was: {}.", secret_number);
            break;
        }
    }
}

fn read_user_input() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}
