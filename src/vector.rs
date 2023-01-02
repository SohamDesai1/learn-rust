fn main(){
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec.len());
    println!("{:?}", vec.capacity());
    vec.is_empty();
    println!("{:?}", vec.contains(&6));
    vec.push(6);
    vec.pop();
    vec.remove(0);
    vec.insert(0, 1);
    vec.swap(0, 1);
    vec.reverse();
    vec.dedup();
    println!("{:?}", vec);
    vec.clear();

}