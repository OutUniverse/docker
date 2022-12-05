#[cfg(test)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
enum Message {
    Hello { id: i32 }
}

fn main() {
    
}

#[cfg(test)]
fn testAtGiveValueToVariable() {
    match 12 {
        num @ (12 | 13) => println!("{}", num),
        _ => println!("Other"),
    }
}

#[cfg(test)]
fn testAtForNewVariable() {
    let msg = Message::Hello {id: 32};

    match msg {
        Message::Hello { id: id @ 1..=21 } => {
            println!("id is {}", id);
        },
        _ => {
            println!("nothing");
        }
    }
}

#[cfg(test)]
fn testOwnerForMatchMode() {
    let s = Some(String::from("test"));

    if let Some(x) = &s {
        println!("{}", x);
    }

    println!("{:?}", s);
}

#[cfg(test)]
fn testDeconstruct() {
    let p = Point { x: 1, y: 2 };

    let Point {x: a, y: b}= p;

    println!("x is {}, y is {}", a, b);
}

#[cfg(test)]
fn testWhileLet() {
    let mut a = Vec::new();

    let b = ["One", "Two", "Three", "Four", "Five"];

    for i in b {
        a.push(i);
    }

    while let Some("Five") = a.pop() {
        println!("This is Five");
    }

    println!("{:#?}", a);
}

#[cfg(test)]
fn testIfLet() {
    let a = Some("test if let");

    if let Some("test if let") = a {
        println!("a is {:?}", a);
    }   
}

#[cfg(test)]
fn testMatch() {
    let mut a = Some(1);

    a = match a {
        Some(i) => Some(i + 1),
        None => None,
    };

    println!("{:?}", a);
}
