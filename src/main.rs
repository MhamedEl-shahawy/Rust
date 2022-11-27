use rand::Rng;
use std::io;
fn main() {
    println!("guess, num!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
