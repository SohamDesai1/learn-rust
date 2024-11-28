trait Summary {
    fn summarize(&self) -> String;
}

trait Fix {
    fn fix(&self) -> String {
        return format!("This is fix");
    }
}
struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!(
            "The name is {} {} and age {}",
            self.first_name, self.last_name, self.age
        );
    }
}

impl Fix for User {}

fn notify(input: impl Summary) {
    println!("{}", input.summarize())
}

// A generic which has to be satisfied by 2 traits
fn fix<T: Summary + Fix>(input: T) {
    println!("{}", input.fix());
}

fn main() {
    let user = User {
        first_name: String::from("Soham"),
        last_name: String::from("Desai"),
        age: 22,
    };

    println!("{}", user.summarize());
    notify(user);

    let user2 = User {
        first_name: String::from("Soham"),
        last_name: String::from("Desai"),
        age: 22,
    };
    fix(user2);
}
