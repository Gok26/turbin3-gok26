use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100); // generating a random number using rand lib

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // initiating the guess with new string datatype

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");  // using io lib get input from the user

        let guess: u32 = match guess.trim().parse() {   // trim the whitespaces and parse converts string to u32
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {   // comparing the user guess with secret number randomly generated.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}