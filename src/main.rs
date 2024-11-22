// import module io(input/output)
// read input from user or write output to console
// additional information (use syntax used for import library is not in prelude in rust)
use std::io;
// rand defines that random number generator
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // rand::tread_rng function that gives use the particular random number
    // gen_range a range expression as an argument and generates random number in the range
    let secret_number = rand::thread_rng().gen_range(1..200);

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    // let for declaration variable
    // mut for mutable variable
    // guess is named variable
    // string::new() created string blank
    // Conclusion crate variable to store the user input
    let mut guess = String::new( );

    // io::stdin() get input from standard input (keyboard)
    // .read_line(&mut guess) read input until press enter
    // .expect if error throw message 'Failed to read line'
    // {guess} variable from input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
