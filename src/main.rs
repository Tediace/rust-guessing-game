// import module io(input/output)
// read input from user or write output to console
// additional information (use syntax used for import library is not in prelude in rust)
use std::io;
// rand defines that random number generator
use rand::Rng;
// another enum and has the variants Less, Greater, and Equal.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // rand::tread_rng function that gives use the particular random number
    // gen_range a range expression as an argument and generates random number in the range
    let secret_number = rand::thread_rng().gen_range(1..200);

    // println!("The secret number is: {}", secret_number);
    // loop keyword created an infinite loop
    loop {
        println!("Please input your guess.");

        // let for declaration variable
        // mut for mutable variable
        // guess is named variable
        // string::new() created string blank
        // Conclusion crate variable to store the user input
        let mut guess = String::new();

        // io::stdin() get input from standard input (keyboard)
        // .read_line(&mut guess) read input until press enter
        // .expect if error throw message 'Failed to read line'
        // {guess} variable from input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // created new variable named guess
        // rust allow us to shadow the previous value of guess with a new one
        // shadowing lets us reuse the guess variable name rather than forcing us to create two unique var
        // the trim method on a string instance will eliminate any whitespace at the beginning and end
        // if input 5 and presses enter, guess looks like this: 5\n
        // the \n represents "new line"
        // trim method eliminates \n or \r\n resulting in just 5
        // SUMMARY parse method on string converts a string to another type
        // the colon (:) after guess tells rust we'll annotate the variable type
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // ok(num) => num is if parsing success num return into var guess
        // err(_) => if parsing failed next to iterating in loop
        // underscore _ is a catchall value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        // match expression to decide what to do next based on which variant of Ordering

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
            //adding break after 'You win!' make program exit loop
            break;
            }
        }
    }
}
