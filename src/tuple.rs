fn main(){
    let tuple: (u8, i8, f32, bool, char, &str) = (255, -128, 3.14, true, 'a', "Hello, world!");
    println!("Tuple: {:?}", tuple);
    println!("First element: {}", tuple.0);

    let (a, b, c) = tuple;
    println!("First element: {}", a);
    println!("Second element: {}", b);
    
}