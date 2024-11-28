fn main() {
    let ans;
    let str1 = String::from("Soham");
    {
        let str2 = String::from("dehdfjkehfwk");
        ans = longest(&str1, &str2);
        println!("{}", ans)
    }
    // println!("{}", ans) // Error as str2 lifetime not long enough
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}
