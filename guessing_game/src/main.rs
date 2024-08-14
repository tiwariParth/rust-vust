use core::num;
use std::{cmp::Ordering, io::{self}};

use rand::Rng;

fn main(){
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read Line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_) => continue,
        }; 

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Too Big")
        }
    }


}