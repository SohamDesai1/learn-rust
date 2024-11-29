use std::io;

fn main() {
    println!("Please input your number 1:");

    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: u8 = match num1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Please input your number 2:");

    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: u8 = match num2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Please select the operation you would like to perform:");
    println!("1) Add");
    println!("2) Subtract");
    println!("3) Multiply");
    println!("4) Divide");

    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read line");
    let ops: u8 = match op.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 4.");
            return;
        }
    };

    match ops {
        1 => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
        2 => println!("Result: {} - {} = {}", num1, num2, num1 - num2),
        3 => println!("Result: {} * {} = {}", num1, num2, num1 * num2),
        4 => {
            if num2 == 0 {
                println!("Division by zero is not allowed.");
            } else {
                println!("Result: {} / {} = {}", num1, num2, num1 / num2);
            }
        }
        _ => println!("Invalid option. Please select a number between 1 and 4."),
    }
}
