fn main(){
    let a = MyEnum::A;
    let b = MyEnum::B(String::from("Hello"));
    let c = MyEnum::C(1, String::from("Hello"));
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}   

#[derive(Debug)]
enum MyEnum {
    A,
    B(String),
    C(i32, String),
}