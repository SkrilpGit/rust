use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // let mutable variable = guess which is a String.new()

    io::stdin()
        .read_line(&mut guess) // read_line takes terminal input and appends to a string,
                               // we pass a reference (&) of our guess variable, making sure,
                               // it is mutable (&mut) so that we can change it
        .expect("Failed to read line!"); // handle the Result of the .readline() if the result is,
                                         // Err crash and display the indicated string

    println!("You guessed: {guess}") // include a name within the braces to reference a variable
                                     // leave it empty and add another parameter for expressions
                                     // i.e. println!("2 + 2 = {}", 2+2)
}
