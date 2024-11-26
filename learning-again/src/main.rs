enum Moves {
    // Type , power
    Punch(String, u8),
    Jab(String, u8),
    Kick(String, u8),
    Block(String, u8),
}

fn main() {
    let punch = Moves::Punch(String::from("attack"), 20);
    let jab = Moves::Jab(String::from("attack"), 40);
    let kick = Moves::Kick(String::from("attack"), 35);
    let block = Moves::Block(String::from("defense"), 20);
    calc(punch);
    calc(jab);
    calc(kick);
    calc(block);
}

fn calc(moves: Moves) {
    match moves {
        Moves::Block(a, b) => println!("This is an {} move with power {}", a, b),
        Moves::Punch(a, b) => println!("This is an {} move with power {}", a, b),
        Moves::Jab(a, b) => println!("This is an {} move with power {}", a, b),
        Moves::Kick(a, b) => println!("This is an {} move with power {}", a, b),
    }
}
