fn main() {
    println!("Hello, world!");
    let number: u32 = 42;
    println!("Number: {}", number);
    println!("Is even: {}", is_even(number));
}

pub fn is_even(number: u32) -> bool {
    // in rust for return boolean value we can use this syntax in which we dont put semicolon
    number % 2 == 0
}
    