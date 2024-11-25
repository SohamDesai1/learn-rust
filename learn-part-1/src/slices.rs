fn main() {
    let array: [u8; 5] = [1, 2, 3, 4, 5];
    let slice: &[u8] = &array[1..3];
    borrowing_slice(array, slice);
}
fn borrowing_slice(arr: [u8; 5], slice: &[u8]) {
    println!("Array: {:?}", arr);
    println!("Slice: {:?}", slice);
    println!("Length of array: {}", arr.len());
    println!("Length of slice: {}", slice.len());
}