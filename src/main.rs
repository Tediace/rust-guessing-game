//import module io(input/output)
//read input from user or write output to console
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    //let for declaration variable
    //mut for mutable variable
    //guess is named variable
    //string::new() created string blank (kosong)
    let mut guess = String::new();

    //io::stdin() get input from standard input (keyboard)
    //.read_line(&mut guess) read input until press enter
    //.expect if error throw message 'Failed to read line'
    //{guess} variable from input
    //
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
