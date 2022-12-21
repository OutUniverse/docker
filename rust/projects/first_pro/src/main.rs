use std::vec;

#[derive(Debug, PartialEq)]
enum Type {
    A(i32),
    B(String),
}

struct Animal {
    name: String,
    age: i32,
}

struct Car {
    name: String,
    age: i32,
}

trait Com {
    fn show(&self) -> String;
}

impl Com for Car {
    fn show(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

impl Com for Animal {
    fn show(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }   
}

fn main() {
    test_struct();
    let a = [1, 2, 3];
    let b: Vec<i32> = a.into();
}

fn test_struct() {
    let v: Vec<Box<dyn Com>> = vec![
        Box::new(Car {name: String::from("Car"), age: 1}), 
        Box::new(Animal {name: String::from("Animal"), age: 2}),
    ];

    for i in &v {
        println!("{}", i.show());
    }

    let mut v2 = Vec::new();
    v2.extend(v);
}

#[cfg(test)]
fn test_iter() {
    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i = *i + 1;
    }

    println!("{:?}", v);
}

#[cfg(test)]
fn test_get() {
    let v1 = vec![
        Type::A(1),
        Type::B(String::from("Lalala")),
    ];

    match v1.get(1) {
        Some(Type::A(_)) => println!("{:?}", v1.get(1)),
        _ => println!("{:?}", v1.get(1)),
    }

    println!("{:?} {:?}", v1.get(0), v1.get(1));
}

#[cfg(test)]
fn test_index() {
    let v1 = vec![
        Type::A(1),
        Type::B(String::from("Lalala")),
    ];

    match v1[0] {
        Type::A(a) => println!("{}", a),
        _ => println!("{:?}", v1[0]),
    }

    println!("{:?} {:?}", v1[0], v1[1]);
}
