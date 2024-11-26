fn main() {
    let ans = is_even(202);
    print!("{}", ans);
}

fn is_even(num: u32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
