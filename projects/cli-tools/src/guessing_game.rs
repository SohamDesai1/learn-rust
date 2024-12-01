use rand::{self, Rng};
use std::io;
fn main() {
    println!("<----------Guessing game---------->");

    let secret_num = rand::thread_rng().gen_range(0..101);

    let mut count: u8 = 0;

    loop {
        println!("Enter a number from 1 to 100:");

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: u8 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return;
            }
        };

        match num.cmp(&secret_num) {
            std::cmp::Ordering::Less => {
                println!("Too small, try higher");
                count = count + 1;
            }
            std::cmp::Ordering::Equal => {
                count = count + 1;
                println!("You guessed it correctly! You took {count} times to guess.");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Too high, try smaller");
                count = count + 1;
            }
        }
    }
}
