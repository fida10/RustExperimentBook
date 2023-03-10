// Chapter 2 (Guessing Game)

// Learing let , match methods

// cargo new, takes the name of the project as the frist argument.

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess...");
        // let used to create a variable. rust variables are immutable by default.
        let mut guess = String::new(); // new instace of String
                                       // process the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        // Compare number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed number: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
