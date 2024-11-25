fn main() {
    let str: &str = "Hello World";
    let mut string: String = String::from("Hello World");
    let slice = &string[0..5];
    slice.len();
    string.push_str("foo");
    string.push('!');
    string.pop();
    string.insert(0, 'H');
    string.remove(0);
    string =  string.replace("Hello", "Hi");
    string.split_whitespace();
    println!("{}", string);
    string.clear();
    
}