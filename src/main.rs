/*
 * My upgraded guessing game, from the Rust book.
 */

//use std::io;
use std::io::{self, Write};
use std::cmp::Ordering;
use std::process::exit;
use rand::Rng;

fn get_user_input(user_message: &str, allowed_range_lower: u16) -> u16 {
	loop {
		//println!( "{user_message}" );
		print!( "{user_message}: " );
		io::stdout().flush().unwrap();	// Flush the output buffer.
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
			println!("Number out of range; must be greater than {allowed_range_lower}.  Please try again.");
			continue;
		} else {
			return user_input;
		}
	}
}


fn main() {
	println!("Guess the number! (enter 'q' to quit)");

	let lower_limit = get_user_input("Enter lower limit", 0);
	let upper_limit = get_user_input("Enter upper limit", lower_limit);

	//let secret_number = rand::thread_rng().gen_range(1..=100);
	let secret_number = rand::thread_rng().gen_range(lower_limit ..= upper_limit);
	//println!("The secret number is: {secret_number} (shhh)");

	//let mut guess;	// Has to be a string (not an int) because we are getting user input.
	let mut guess: u16;	// Using our get_user_input() function, must be an int.
	loop {
		//guess = "".to_string();
		// because we're accepting input from the user, and read_line() below returns a string.
		//println!("Please input your guess (a number between {lower_limit} and {upper_limit}, or 'q' to quit).");
		/*
		io::stdin()
			.read_line(&mut guess)			// Includes the \n
			.expect("ERR; Failed to read input.");		// expect() returns the value entered by the user if the Result enum is 'Ok',
			// otherwise (Result enum is 'Err'), it outputs the passed string and crashes.
		*/

		// Let's convert this to use our get_user_input() function.
		guess = get_user_input(&format!("Please input your guess (a number between {} and {}, or 'q' to quit)", lower_limit, upper_limit), lower_limit);

		// Don't need this check any more, it is performed in our get_user_input() function.
		/*if guess.trim().to_lowercase() == "q" {
			//println!("Quitting.");
			break;
		}*/

		// Don't need this either, now that we are using our get_user_input() function.
		//let guess: u32 = guess.trim().parse().expect("ERR; Please type a number.");
		/*let guess: u16 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => { println!("Number only, please."); continue; },
		};*/
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
