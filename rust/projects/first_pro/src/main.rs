use std::fmt::Display;

// use std::fmt::*;

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(" "))
    }
}

fn main() {
    let a = Wrapper(vec!["hello".to_string(), "world".to_string()]);

    println!("{}", a);
}
