fn even(v: Vec<i32>) -> Vec<i32> {
    let mut nv: Vec<i32> = Vec::new();
    for val in v {
        if val % 2 == 0 {
            nv.push(val);
        }
    }

    return nv;
}
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(4);
    println!("{:?}", even(vec));
    // OR
    let v = vec![1,2,3]; // by the vector macro
    println!("{:?}",v);
}


