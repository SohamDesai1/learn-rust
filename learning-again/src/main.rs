fn main() {
    let name = String::from("Soham");
    let len = get_str_len(name);
    println!("The length is {}",len);
}

fn get_str_len(s: String) -> usize {
    s.chars().count()
}
