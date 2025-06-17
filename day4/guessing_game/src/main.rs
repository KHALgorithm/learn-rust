use std::io;
use rand::Rng;
use std::cmp::Ordering;
// This is a simple guessing game where the user has to guess a randomly generated number between 1 and 100.
fn main() {
    println!("Welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");

    // Generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        
        // Compare the user's guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
        }
        println!("Your guess was: {}", guess);
    }
}
