use rand::Rng; // random number generator
use std::cmp::Ordering;
use std::io; // input output library // for comparing two values

fn main() {
    println!("Guess the number");

    // generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        // for storing the user input
        let mut guess = String::new();
        // to handle user input the read_line just append it not overwrite it
        // this returns two possible states (Result) Ok or Err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        // parse also returns a Result type so we need to handle it with expect
        // shadowing the guess variable
        let guess: u32 = guess.trim().parse().expect("failed to parse"); // 45/enter (so we need to remove the enter so we will use trim())

        println!("You guessed: {guess}");

        // Checking the guess with the secret number using match statement which is similar to switch statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

// fn main() {
//     // let age = 24;
//     // // variables are immutable by default in rust
//     // age = 25;

//     // mutable variables
//     let mut age2=24;
//     age2 = 25;
//     println!("age2 is {age2}");
//     // println!("My age is {age}")

// }
