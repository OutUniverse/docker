use std::ops::Deref;

struct SelfRef<'a> {
    value: String,
    reff: &'a str
}

fn main() {
    let s = String::from("hello world");
    let a = SelfRef {
        value: s,
        reff: &s
    };
}

