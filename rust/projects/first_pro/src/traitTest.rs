use std::{ops::Add, fmt::Display};

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point { x: self.x + p.x, y: self.y + p.y }
    }
}

pub trait AddToSelf<T = Self> {
    fn addToSelf(&mut self, b: &Self) -> &mut Self;
}

impl<T: Add<T, Output = T> + Copy> AddToSelf for Point<T> {
    fn addToSelf(&mut self, b: &Self) -> &mut Self {
        self.x = self.x + b.x;
        self.y = self.y + b.y;

        self
    }
}

fn largest<T: PartialOrd + Copy + Display> (arr: &[T]) -> T {
    let mut largest = arr[0];

    for &item in arr.iter() {
        if largest < item {
            largest = item;
        }
    }

    largest
}

fn main() {
    let p1 = Point {
        x: 1.0,
        y: 1.0,
    };

    let p2 = Point {
        x: 0.5,
        y: 0.5,
    };


    let p3 = p1.add(p2);

    println!("{:?}]", p3);

    let a = [1, 5, 16, 1, 30];

    println!("{}", largest(&a));
}
