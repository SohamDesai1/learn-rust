fn main(){
    let array: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    let array: [u8; 5] = [0; 5];
    println!("Length of array: {}", array.len());
    // slice
    let slice: &[u8] = &array[1..3];
    println!("Slice: {:?}", slice);
    // array of strings
    let array: [&str; 3] = ["Hello", "world", "!"];
    println!("Array of strings: {:?}", array);
    // array of arrays
    let array: [[u8; 2]; 2] = [[1, 2], [3, 4]];
    println!("Array of arrays: {:?}", array);
    // borrowing slice
    let array: [u8; 5] = [1, 2, 3, 4, 5];
    let slice: &[u8] = &array[1..3];
    borrowing_slice(array, slice);
}

fn borrowing_slice(arr: [u8;5], slice: &[u8]) {
    println!("Array: {:?}", arr);
    println!("Slice: {:?}", slice);
    println!("Length of array: {}", arr.len());
    println!("Length of slice: {}", slice.len());

}