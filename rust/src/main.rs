use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Duration; // Import the Duration type
use std::thread; // Import the thread module for sleep

fn main() {
    println!("This is the 'Guess the number' game!");

    let mut end_game = false;

    while !end_game{
        println!("**---------------------------------------------------**");
        println!("I'm thinking of a number between 1 and 10...");
        let secret_number = rand::thread_rng().gen_range(1..=10);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("The secret number was: {}", secret_number);                    
                    println!("**---------------------------------------------------**");
                    thread::sleep(Duration::from_secs(2));
                    break;
                }
            }
        }

        
        loop{
            let mut play_again = String::new();
            println!("Do you want to play again? (yes/no)");

            io::stdin()
                .read_line(&mut play_again)
                .expect("Failed to read line");

            play_again = play_again.trim().to_string();

            println!("You typed: {}", play_again);
            //check if the input was yes or no
            if play_again != "yes" && play_again != "no" {
                println!("Please type yes or no!");
                continue;
            }
            else {
                if play_again != "yes" {
                    end_game = true;
                    println!("Thanks for playing!");
                    println!("Exiting the game...");
                    thread::sleep(Duration::from_secs(2));            
                } else {
                    println!("Starting a new game...");
                }

                break;
            }
        }        
    }
}