use std::fmt::Display;
use std::ops::Add;

#[allow(dead_code)]
type Mixed = Box<dyn Fn() + Send + 'static>;

struct Wrapper(Vec<String>);

struct Meters(u32);

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Meters {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(" "))
    }
}

fn main() {
    let a = Wrapper(vec!["hello".to_string(), "world".to_string()]);

    println!("{}", a);

    println!("1m + 10m = {}", Meters(1) + Meters(10));
}
