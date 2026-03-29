use std::io;
use std::cmp::Ordering;
use std::process::exit;
use rand::Rng;

fn main() {
    println!("Guess the number! (enter 'q' to quit)");

    let mut lower_limit: u32;
    let mut upper_limit: u32;

    loop {
        println!("Enter lower limit.");
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("ERR; Failed to read input.");
        if user_input.trim().to_lowercase() == "q" {
            exit(0);
        }
        lower_limit = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Numbers only, please."); continue; },
        };
        if lower_limit <= 0 {
            println!("Number greater than zero, please.");
            continue;
        } else {
            break;
        }
    }

    loop {
        println!("Enter upper limit.");
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("ERR; Failed to read input.");
        if user_input.trim().to_lowercase() == "q" {
            exit(0);
        }
        upper_limit = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Numbers only, please."); continue; },
        };
        if upper_limit <= lower_limit {
            println!("Upper limit must be greater than lower limit.");
            continue;
        } else {
            break;
        }
    }

    //let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = rand::thread_rng().gen_range(lower_limit ..= upper_limit);
    //println!("The secret number is: {secret_number}");

    let mut guess;  // Has to be a string, not an int,
    loop {
        //guess = String::new();
        guess = "".to_string();
        // because we're accepting input from the user, and read_line() below returns a string.
        println!("Please input your guess (a number between {lower_limit} and {upper_limit}, or 'q' to quit).");
        io::stdin()
            .read_line(&mut guess)      // Includes the \n
            .expect("ERR; Failed to read input.");     // expect() returns the value entered by the user if the Result enum is 'Ok',
            // otherwise (Result enum is 'Err'), it outputs the passed string and crashes.

        if guess.trim().to_lowercase() == "q" {
            //println!("Quitting.");
            break;
        }

        //let guess: u32 = guess.trim().parse().expect("ERR; Please type a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Number only, please."); continue; },
        };
        // In addition to spaces, trim() also removes \n from the end of the string.
        //println!("You guessed: {guess}");
        //println!("You also guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Bingo!  You win!");
                break;
            }
        }
    }
} // end main
