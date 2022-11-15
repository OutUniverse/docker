#[derive(Debug)]
struct User {
    username: String,
    age: u8,
    active: bool,
    sign_in_count: u64,
    email: String,
}

fn main() {
    let u1 = User {
        username: String::from("User 1"),
        age: 8,
        active: true,
        sign_in_count: 10,
        email: String::from("johnson@gmail.com"),
    };

    let u2 = User {
        username: String::from("User 2"),
        ..u1
    };

    // println!("{:?}", u1);
}