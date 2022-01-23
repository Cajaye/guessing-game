use std::io; //we get input/output from the std library

fn main() {
    println!("Guessing game!");
    println!("Enter your guess");

    let mut guess = String::new(); //empty mutale string

    io::stdin().read_line(&mut guess).expect("Cannot read line"); // & references a variable and &mut references a mutabe variable

    println!("You entered : {}", guess)
}
