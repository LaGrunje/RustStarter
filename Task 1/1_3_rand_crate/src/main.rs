use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new(); // let - variable syntaxis, by default all variables are immutable, keyword mut makes it mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failes to read line");

    println!("The secret number is: {secret_number}");
    println!("You guessed: {guess}");
}
