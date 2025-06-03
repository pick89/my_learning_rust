use std::cmp::Ordering;
use std::io;
use rand::Rng;
use colored::*;

const MAX_ATTEMPTS: u32 = 7;

fn main() {
    loop {
        run_game();
        if !ask_restart() {
            print_info("👋 Thanks for playing the guessing game. Goodbye!");
            break;
        }
    }
}

fn run_game() {
    print_header("🎯 Welcome to the guessing game!\n");

    let secret_number = rand::rng().random_range(1..=100);
    print_info(&format!(
        "🤔 I have selected a secret number between 1 and 100. You have {} attempts!",
        MAX_ATTEMPTS
    ));

    let mut guesses = Vec::new();

    for attempt in 1..=MAX_ATTEMPTS {
        print_attempt_prompt(attempt);

        match read_user_input() {
            Some(guess) => {
                println!("🔢 You guessed: {}", guess);
                guesses.push(guess);

                match guess.cmp(&secret_number) {
                    Ordering::Less => print_hint("📉 Too small!"),
                    Ordering::Greater => print_hint("📈 Too big!"),
                    Ordering::Equal => {
                        print_success(&format!(
                            "🎉 Correct! The number was: {} in {} attempts!",
                            secret_number, attempt
                        ));
                        return;
                    }
                }

                print_guesses(&guesses);
            }
            None => print_error("⚠️ Invalid input. Please enter a valid number."),
        }
    }

    print_error(&format!("💥 Game over! The number was: {}.", secret_number));
}

fn read_user_input() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}

fn print_attempt_prompt(attempt: u32) {
    let message = if attempt == MAX_ATTEMPTS {
        format!("⚠️ Last Attempt ({}): Enter your guess (1–100):", attempt)
    } else {
        format!("🔁 Attempt {}: Enter your guess (1–100):", attempt)
    };
    println!("\n{}", message.blue());
}

fn print_guesses(guesses: &[u32]) {
    println!(
        "{}",
        format!("📜 Previous guesses: {:?}", guesses).red().bold()
    );
}

fn ask_restart() -> bool {
    println!("\n💡 Would you like to play again? (yes/no)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");
    response.trim().eq_ignore_ascii_case("yes")
}

// --- Helper Printers ---
fn print_success(msg: &str) {
    println!("{}", msg.green().bold());
}

fn print_info(msg: &str) {
    println!("{}", msg.yellow());
}

fn print_error(msg: &str) {
    println!("{}", msg.red());
}

fn print_hint(msg: &str) {
    println!("{}", msg.cyan());
}

fn print_header(msg: &str) {
    println!("{}", msg.bold().underline());
}
