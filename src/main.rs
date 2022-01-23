use rand::Rng;
use std::io; //we get input/output from the std library // Rng defines methods that random number generators implement

fn main() {
    println!("Guessing game!");
    println!("Enter your guess");

    //let secret_number =

    let mut guess = String::new(); //empty mutable string

    io::stdin().read_line(&mut guess).expect("Cannot read line"); // & references a variable and &mut references a mutabe variable

    println!("You entered : {}", guess)
}
