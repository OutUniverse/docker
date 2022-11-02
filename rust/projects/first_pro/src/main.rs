fn main() {

    // &str to string
    let s1 = "hello".to_string();
    let s2 = String::from("hello");

    println!("{} {}", s1, s2);

    // string to &str

    let s3 = &s2;
    let s4 = &s2[..];
    let s5 = s2.as_str();

    println!("{} {} {}", s3, s4, s5);

}
