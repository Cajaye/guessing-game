use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a guessing game!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your guess!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read input");

        let guess: u32 = match guess.trim().parse() {
            //call match to run code conditionally
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); //tell the player what their guess is

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
