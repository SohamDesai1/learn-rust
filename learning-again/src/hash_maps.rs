use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    users.insert(1, String::from("user1"));
    users.insert(2, String::from("user2"));

    let demo_user = users.get(&4);

    match demo_user {
        Some(d) => println!("{}",d),
        None => println!("Not Found")
    }
}

fn group_by(v: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, val) in v {
        hm.insert(key, val);
    }
    return hm;
}