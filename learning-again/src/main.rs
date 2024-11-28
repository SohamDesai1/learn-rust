use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut sum = 0;
            for j in i * 10000..(i + 1 * 10000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap()
        });
    }
    drop(tx);
    let mut ans = 0;
    for val in rx {
        ans = ans + val;
    }

    println!("{}", ans);
}
