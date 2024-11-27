fn print_str(s:&mut String) {
    s.push_str(" desai");
    println!("{}",s)
}

fn main() {
    let mut s = String::from("soham");
    print_str(&mut s);

    //OR
    let mut a =  String::from("Soham Desai");
    let b = &mut a;
    // let c = &mut a; // Gives error because the rule says that no 2 mutable var 
    let c = &b;
    println!("{},{}",b,c)
}
