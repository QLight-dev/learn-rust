use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let answer = rand::rng().random_range(1..=100);

    loop {
        println!("Input Guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&answer) {
            Ordering::Less => println!("Your guess is too small."),
            Ordering::Greater => println!("Your guess is too big."),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
}
