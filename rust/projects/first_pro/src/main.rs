fn main() {
    let a = vec![1, 2, 3, 4, 5];

    {
        let result = match IntoIterator::into_iter(a) {
            mut i => loop {
                match i.next() {
                    Some(x) => {println!("{}", x)},
                    None => break
                }
            }
        };

        print!("{:?}", result);
    }
}

#[cfg(test)]
fn test_next() {
    let a = vec![1, 2, 3, 4, 5];

    let mut b = a.into_iter();

    println!("{:?}", b.next());
    println!("{:?}", b.next());
    println!("{:?}", b.next());
    println!("{:?}", b.next());
    println!("{:?}", b.next());
}
