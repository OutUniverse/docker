#[derive(Debug)]
struct SelfRef<'a> {
    value: String,
    reff: Option<&'a str>
}

impl <'a> SelfRef<'a> {
    fn tie_self(&'a mut self) {
        self.reff = Some(&self.value);
    }
}

fn main() {
    let mut a = SelfRef { value: "a".to_string(), reff: None };

    a.tie_self();

    println!("{:?}", a);
}
