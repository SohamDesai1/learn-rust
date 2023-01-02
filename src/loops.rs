fn main() {
    let a = 3;
    if a > 0 {
        println!("a is positive");
    } else if a < 0 {
        println!("a is negative");
    } else {
        println!("a is zero");
    }
    for i in 1..4 {
        println!("i is {}", i);
    }
    let mut i = 1;
    while i < 4 {
        println!("i is {}", i);
        i += 1;
        if i == 3 {
            println!("i is 3, break");
            break;
        }
    }
    let i = 1;
    loop {
        println!("i is {}", i);
        if i == 3 {
            println!("i is 3, break");
            break;
        }
    }
    let i = 5;
    match i {
        1 => println!("i is 1"),
        1..=5 => println!("i is between 1 and 5"),
        1 | 2 => println!("i is 1 or 2"),
        _ => println!("i is something else"),
    }

}