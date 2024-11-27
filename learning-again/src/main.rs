use std::fs::read_to_string;

fn main() {
    let res = read_to_string("demo.txt");

    match res {
        Ok(data) => println!("{:?}",data),
        Err(err) => println!("Error occured {}",err)
    }

}

