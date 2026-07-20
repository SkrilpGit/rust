use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("the secret number is: {}",secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // let mutable variable = guess which is a String.new()

        
        io::stdin()
            .read_line(&mut guess) // read_line takes terminal input and appends to a string,
                                   // we pass a reference (&) of our guess variable, making sure,
                                   // it is mutable (&mut) so that we can change it
            .expect("Failed to read line!"); // handle the Result of the .readline() if the result is,
                                             // Err crash and display the indicated string
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");// include a name within the braces to reference a variable
                                         // leave it empty and add another parameter for expressions
                                         // i.e. println!("2 + 2 = {}", 2+2)
    
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
