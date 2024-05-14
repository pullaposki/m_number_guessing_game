use std::io;
use std::cmp::Ordering;
use rand::Rng;

// extern crate rand;

fn main() {
    let mut rng = rand::thread_rng().gen_range(1..=100);
    let mut max_number = 100;
    let mut min_number = 1;

    loop {
        println!("Guess the number between {} and {}!", min_number, max_number);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let formatted_guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(msg) => {
                println!("Error: {}", msg);
                continue;
            }            
        };

        if formatted_guess < min_number || formatted_guess > max_number {
            println!("The number is out of range!");
            continue;
        }

        match formatted_guess.cmp(&rng) {
            Ordering::Less => {
                println!("Too small! Lowest number now is {}", formatted_guess);
                min_number = formatted_guess;
            }
            Ordering::Greater => {
                println!("Too big! Highest number now is {}", formatted_guess);
                max_number = formatted_guess;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        
    }
}
