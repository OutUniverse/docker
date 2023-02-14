fn main() {
    
}

#[cfg(test)]
fn test_enum_and_fold_on_iter() {
    let a = vec![1, 2, 3, 4];

    let val = a.iter()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        .fold(0, |sum, acm| sum + acm);

    println!("{}", val);
}

#[cfg(test)]
fn test_impl_iterator_for_custom_data() {
    struct Counter {
        count: u32
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            match self.count {
                0..=5 => {
                    self.count += 1;
                    Some(self.count)
                },
                _ => None,
            }
        }
    }

    let mut a = Counter::new();

    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
}

#[cfg(test)]
fn test_filter_colection() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Shoe {
        size: f32,
        style: String
    }

    let a = vec![
        Shoe {size: 32.0, style: "a".to_string()},
        Shoe {size: 42.0, style: "b".to_string()},
        Shoe {size: 42.5, style: "c".to_string()},
    ];

    let c: Vec<_> = a.into_iter().filter(|item| item.size == 42.5).collect();

    println!("{:?}", c);
}

#[cfg(test)]
fn test_zip_collect() {
    use std::collections::HashMap;

    let names = ["a", "b"];
    let ages = [1, 2];

    // let combine = names.into_iter().zip(ages.into_iter());
    let combine: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", combine.get("a"));
    for i in combine {
        println!("{:?}", i);
    }
}

#[cfg(test)]
fn test_all_iter_type() {
    let a = vec![1, 2, 3, 4];

    // into_iter move the a
    let b = a.into_iter();

    println!("{:?}", b);

    let a = vec![1, 2, 3, 4];

    // iter return a ref
    let b = a.iter();

    // i is the ref
    for i in b {
        println!("{}", i);
    }

    let mut a = vec![1, 2, 3, 4];

    // iter_mut return a mut ref
    let b = a.iter_mut();

    // item is the mut ref
    for i in b {
        *i = 1;
    }

    println!("{:?}", a);
}

#[cfg(test)]
fn iter_impl_into_iterator() {
    let a = vec![1, 2, 3, 4, 5];

    for i in a.into_iter().into_iter().into_iter().into_iter() {
        println!("{}", i);
    }
}

#[cfg(test)]
fn use_match_achieve_loop() {
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
