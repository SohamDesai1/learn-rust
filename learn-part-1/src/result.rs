fn divide(divident: i32, divisor: i32) -> Result<i32, MyError> {
    if divident % divisor != 0 { Err(MyError::Error) } else { Ok(divident / divisor) }
}

fn main() {
    let div1 = divide(10, 2);
    let res = div1.expect("div1 is error");

    // first way to match
    match div1 {
        Ok(x) => println!("div1 is {}", x),
        Err(e) => println!("div1 is error {}", e),
    }

    // second way to match
    if div1.is_ok() {
        println!("div1 is ok");
    } else {
        println!("div1 is error");
    }

    // third way to match
    println!("{}", div1.unwrap());
    println!("{}", div1.unwrap_or("NT")); //if div1 is error, return NT
}