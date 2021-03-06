// Exercise: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);
}
