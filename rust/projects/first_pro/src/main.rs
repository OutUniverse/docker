#[derive(Debug)]
struct SelfRef<'a> {
    value: String,
    reff: Option<&'a str>
}

fn main() {
    let s = String::from("hello world");
    let mut a = SelfRef {
        value: s,
        reff: None
    };

    a.reff = Some(&a.value);

    println!("{:?}", a);
}

