use std::io::{self, Read};

fn main(){
    println!("guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read Line");

}