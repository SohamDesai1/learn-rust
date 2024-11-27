fn main() {
    let word = String::from("Soham Desai");
    let ans = find_first(&word);
    println!("{}",ans)
}

fn find_first(word: &String) -> &str {
    let mut index = 0;
    for (_,val) in word.chars().enumerate() {
        if val == ' ' {
            break;
        }
        index += 1;
    }
    return &word[0..index];
}
