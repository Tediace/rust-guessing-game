// import module io(input/output)
// read input from user or write output to console
// additional information (use syntax used for import library is not in prelude in rust)
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // let for declaration variable
    // mut for mutable variable
    // guess is named variable
    // string::new() created string blank
    // Conclusion crate variable to store the user input
    let mut guess = String::new( );

    let apples = 5; // immutable
    let mut bananas = 5; // mutable

    // io::stdin() get input from standard input (keyboard)
    // .read_line(&mut guess) read input until press enter
    // .expect if error throw message 'Failed to read line'
    // {guess} variable from input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
