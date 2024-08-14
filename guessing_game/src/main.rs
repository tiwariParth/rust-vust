use std::io::{self};

fn main(){
    println!("guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read Line");

    println!("You guessed: {}",guess);


}