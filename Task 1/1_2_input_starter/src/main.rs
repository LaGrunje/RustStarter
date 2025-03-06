use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // let - variable syntaxis, by default all variables are immutable, keyword mut makes it mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failes to read line");

    println!("You guessed: {guess}");
}
