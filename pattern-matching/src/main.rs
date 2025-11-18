use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, let's start the game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess_number: String = String::new();

    loop {
        guess_number.clear();
        println!("Please input your guess number: ");
        io::stdin().read_line(&mut guess_number).expect("Failed to read line");

        println!("Your guessed number is: {}", guess_number.trim());

        match guess_number.trim().parse::<u32>() {
        
            Ok(good) => { 
                match good.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win! \n The secret number is: {}", secret_number);
                        break;
                    }
                }
            }

            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        }
    }
}
