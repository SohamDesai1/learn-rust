use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 0..500 {
            println!("dwd {}", i);
        }
    });

    for i in 0..50 {
        println!("as2q {}", i)
    }

    let v = vec![1, 2, 3];
    // move key to pass ownership of v as it can go out of scope  
    thread::spawn(move || println!("{:?}", v));
}
