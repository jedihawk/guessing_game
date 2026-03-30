use std::io;
use std::cmp::Ordering;
use std::process::exit;
use rand::Rng;

fn get_user_input(user_message: &str, allowed_range_lower: u16) -> u16 {
    loop {
        println!( "{user_message}" );
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).expect("ERR; Failed to read input.");
        if user_input.trim().to_lowercase() == "q" {
            exit(0);
        }
        let user_input: u16 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Numbers only, please."); continue; },
        };
        if user_input <= allowed_range_lower {
            println!("Number out of range.  Try again.");
            continue;
        } else {
            return user_input;
        }
    }
}


fn main() {
    println!("Guess the number! (enter 'q' to quit)");

    let lower_limit = get_user_input("Enter lower limit.", 0);
    let upper_limit = get_user_input("Enter upper limit.", lower_limit);

    //let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = rand::thread_rng().gen_range(lower_limit ..= upper_limit);
    //println!("The secret number is: {secret_number} (shhh)");

    let mut guess;  // Has to be a string (not an int) because we are getting user input.
    loop {
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
        let guess: u16 = match guess.trim().parse() {
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
