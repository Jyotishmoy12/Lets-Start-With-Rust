use std::io; // input output library

fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // for storing the user input
    let mut guess = String::new();
    // to handle user input the read_line just append it not overwrite it
    // this returns two possible states (Result) Ok or Err
    io::stdin().read_line(&mut guess).expect("Failed to read user input");

    println!("You guessed: {guess}");
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
