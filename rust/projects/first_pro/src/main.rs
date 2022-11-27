use std::io;

fn main() {
    let a = [1; 5];
    println!("Please input index:");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read");

    let index: usize = index.trim().parse().expect("Index was not a number");

    let element = a[index];

    println!("The index {} value is {}", index, element);
}
