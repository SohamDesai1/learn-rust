fn create_string() {
    let mut a = String::from("soham");
    a = print_str(a);
    println!("{}", a);
}

fn print_str(s2: String) -> String {
    println!("{}", s2);
    return s2;
}
fn main() {
    create_string();
}