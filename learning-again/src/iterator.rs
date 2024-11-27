fn main() {
    let a = vec![1, 3, 4];
    // using for loop
    for val in a {
        println! {"{}",val};
    }

    // using iter --> for immutable
    let b = vec![1, 3, 4];
    let b_iter = b.iter();
    for val in b_iter {
        println!("{}", val)
    }

    // mutable iters
    let mut c = vec![1, 2, 3];
    let c_iter = c.iter_mut();
    for val in c_iter {
        *val = *val + 1;
    }

    // OR
    let mut c_iter = c.iter_mut();
    while let Some(val) = c_iter.next() {
        println!("{}", val);
    }

    // into iter --> takes the ownership rather than borrowing
    let d = vec![2, 4, 6];
    let d_iter = d.into_iter();
    for val in d_iter {
        println!("{}", val);
    }

    // Consuming Adaptors --> consumes the iterator and not the vector
    let e = vec![2, 3, 1];
    let e_iter = e.iter();
    let total: i32 = e_iter.sum();
    println!("{}", total);

    // Iterator adaptors --> does not consume but changes some aspect of the iter
    let f = vec![1, 4, 6];
    let f_iter1 = f.iter();
    let f_iter2 = f_iter1.map(|x| x * 2);
    for val in f_iter2 {
        println!("{}", val);
    }
    let f_iter1 = f.iter();
    let f_iter3 = f_iter1.filter(|x| *x % 2 == 0);
    for val in f_iter3 {
        println!("{}", val);
    }
}
