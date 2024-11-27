fn main() {
    let index = find_first_a(String::from("Soham"));
    match index {
        Some(val) => println!("{}", val+1),
        None => println!("Not found"),
    }
}

fn find_first_a(s: String) -> Option<u8> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as u8);
        }
    }
    return None;
}
