use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_fn(i).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_fn(i: i32) {
    let s1 = demo().await;
    println!("First {s1} and {i}");
    let s2 = demo().await;
    println!("Second {s2} and {i}");
}
async fn demo() -> String {
    sleep(Duration::from_millis(100)).await;
    "Demo".to_owned()
}
