use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {
        let mut guess = String::new();  // Has to be a string, not an int,
        // because we're accepting input from the user, and read_line() below returns a string.
        println!("Please input your guess (a number between 1 and 100, 'q' to quit).");
        io::stdin()
            .read_line(&mut guess)      // Includes the \n
            .expect("Failed to read line");     // expect() returns the value entered by the user if the Result enum is 'Ok',
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
