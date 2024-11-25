fn divide(divident:i32, divisor:i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(divident / divisor)
    }
}


fn main(){
    let div1 = divide(10, 2);
    let div2 = divide(10, 0);
    println!("{:?} unwraps {}", div1,div1.unwrap());
    // println!("{:?} unwraps {}", div2,div2.unwrap());
}