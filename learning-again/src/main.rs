struct User {
    name: String,
    lastname: String,
    age: u16,
}

fn main() {
    let user = User {
        name: String::from("Soham"),
        lastname: String::from("Desai"),
        age: 22,
    };

    println!("{}",user.name);
    println!("{}",user.lastname);
    println!("{}",user.age)
}
