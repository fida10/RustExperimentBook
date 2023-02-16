use std::io; //line to import module 
use std::cmp::Ordering;
use rand::Rng;

pub fn guessing_game(){
       //println!("The secret number is {}", secret_number);

       println!("Guess a number!"); 
       let secret_number = rand::thread_rng().gen_range(1..=100);
       /*
       * Using "thread_rng" method to generate a number between 1 and 100
       * Range is specified in the gen_range() function
        */

       loop{
        println!("Input your guess: "); //print statements

        let mut guess = String::new();
        /*
        * Create a variable in Rust with "let" keyword
        * "mut" means the variable is changeable. All variables are final by default
        * "guess" is the name of the variable
        * "String" is the type of variable; in this case, a string.
        * new() creates a new, empty String. 
        * This will not work with comparison to integers. We must convert to an integer, which we do below
        */
        
        io::stdin() //using the input function Stdin
            .read_line(&mut guess) //this is the line that actually gets input from the user (read_line function)
            //& indicates that this "guess" is not a variable but a reference
            //read_line can return either OK or Err. This return value is called an enum, a type that can take multiple forms
            .expect("Failed to read line");
            //kind of like a try/catch block. This expect will trigger if read_line returns "Err"
            //This is helpful for debugging code as it gives us an error message.

            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
            //Ok executes if user inputs a number
            //Err executes if user inputs a non number, will continue with the program if Err is hit
        //must be put after input as before, it is null

        println!("You guessed: {}", guess);
        //we cannot print variables directly. In order to print "guess", we use substitution
        //This involves the paranthesis. Values placed after the comma go into the paranthesis
        //So this will print "You guessed <guess>"
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }
}