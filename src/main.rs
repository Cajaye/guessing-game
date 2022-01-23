use rand::Rng;
use std::io;

fn main() {
    println!("This is a guessing game!");
    println!("Enter your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read input");
    println!("You guessed: {}", guess);
}
