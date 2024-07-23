//
// Chapter 2 Guessing Game
//

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Let us play the GUESS THE NUMBER! game!");

    let mut no_of_guesses: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Why do I have to declare this for every cycle?

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guessprime: i32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}, but the real number is {}", guessprime, secret_number);

        match guessprime.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        no_of_guesses = no_of_guesses + 1;
        if no_of_guesses > 7 {
            break;
        } else {
            println!("You only have {} guesses left", 7 - no_of_guesses)
        }

        let get_out_of_here = 0;

        if guessprime < get_out_of_here {
            println!("OOps! YOu entered a negative number -- good bye!!!");
            break;
        }

    }
}