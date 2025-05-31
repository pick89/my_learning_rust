# 🧠 Rust Guessing Game – Practice Exercises

These exercises build on your number guessing game to help you improve your Rust coding skills, step by step.

---

## 🟢 Level 1: Core Practice (Beginner)

1. **Limit the Number of Attempts**
   - Only allow the user to guess 7 times.
   - If they don't guess correctly, end the game with:
     ```
     💥 Game Over! The number was: <secret_number>
     ```

2. **Restart the Game**
   - After a win or loss, ask:
     ```
     Play again? (y/n)
     ```
   - If `"y"`, restart the game. If `"n"`, exit.

3. **Display Previous Guesses**
   - Keep a list of all the user’s guesses.
   - After each guess, display:
     ```
     Your guesses so far: 15, 44, 63
     ```

---

## 🟡 Level 2: Input & Logic Handling (Intermediate)

4. **Validate Input Range**
   - If the user enters a number outside 1–100, show:
     ```
     ⚠️ Number out of range! Please guess between 1 and 100.
     ```

5. **Show the Difference**
   - After each wrong guess, display how far the guess was from the secret number:
     ```
     You're off by 27!
     ```

6. **Add Difficulty Levels**
   - At the start, ask:
     ```
     Choose difficulty: (1) Easy (2) Medium (3) Hard
     ```
   - Set allowed attempts:
     - Easy → 10
     - Medium → 7
     - Hard → 5

---

## 🔵 Level 3: Structs & Modularization (Advanced)

7. **Use a `Game` Struct**
   - Refactor your game logic into a struct:
     ```rust
     struct Game {
         secret_number: u32,
         attempts: u32,
         max_attempts: u32,
         guesses: Vec<u32>,
     }
     ```

8. **Split Code into Modules**
   - Break your game into multiple files:
     - `main.rs` → entry point
     - `game.rs` → game logic
     - `input.rs` → input handling

9. **Add Unit Tests**
   - Write tests for:
     - Input parsing
     - Guess comparison
     - Attempt tracking

---

## 🔴 Bonus Challenges

10. **Add a Timer**
    - Use `std::time::Instant` to track how long it takes the user to finish.
    - At the end, print:
      ```
      ⏱️ You finished in 32.5 seconds!
      ```

11. **Add Colored Output**
    - Use the [`colored`](https://crates.io/crates/colored) crate.
    - Example:
      ```rust
      println!("{}", "Too big!".red());
      ```

12. **Save High Score**
    - Save best scores to a file (e.g. `highscores.txt`).
    - When the game starts, print:
      ```
      🏆 Best score: 3 attempts
      ```

---

## ✅ How to Use These Exercises

- Try implementing each task on your own.
- Improve your code readability and structure as you go.
- Ask for help or hints if you get stuck.

Happy hacking! 🚀
