fn print_str(s:&String) {
    println!("{}",s)
}

fn main() {
    let s = String::from("soham");
    print_str(&s);
    
    //OR
    let a = String::from("Soham");
    let b = &a;
    println!("{}",a);
    println!("{}",b);
}
