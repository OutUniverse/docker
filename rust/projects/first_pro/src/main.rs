use std::ops::Add;

#[derive(Debug)]
struct Test;

impl Add for Test {
    type Output = Test;
    fn add(self, rhs: Self) -> Self::Output {
        Test
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

fn main() {
    // let t1 = Test{};
    // let t2 = Test{};

    // let t3 = t1 + t2;

    println!("{:?}", Test + Test);
}
