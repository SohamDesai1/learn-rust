use std::collections::HashMap;
fn main(){
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(0, String::from("Hello"));
    map.insert(1, String::from("World"));
    println!("{:?}", map.len());

    match map.get(&0) {
        Some(value) => println!("{:?}", value),
        None => println!("None"),
    }

    map.remove(&0);
    println!("{:?}", map);
    map.contains_key(&0);
    map.clear();
    println!("{:?}", map);
    println!("{:?}", map.is_empty());
}