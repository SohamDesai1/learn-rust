#[tokio::main]
async fn main(){
    let a = my_fn();
    println!("Demo");
    a.await;
}

async fn my_fn() {
    let s1 = demo().await;
    println!("First {s1}");
    let s2 = demo().await;
    println!("Second {s2}");
}
async fn demo() ->String{
    "Demo".to_owned()
}