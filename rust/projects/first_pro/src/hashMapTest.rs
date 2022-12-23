use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let mut a = HashMap::new();

    a.insert("a", 1);

    println!("{:?}", a.get("a"));

    let b = a.entry("b").or_insert(2);

    println!("{:?}", b);

    *b = 3;

    println!("{:?}", a.get("b"));

    println!("{:?}", a["a"]);

    let c = HashMap::from(
        [
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]
    );
}
