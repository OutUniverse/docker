#[derive(Debug)]
#[allow(dead_code)]
struct Test<'a> {
    part: &'a str,
}

#[allow(dead_code)]
fn main() {
    let a = String::from("This is a test. You must believe");
    let b = a.split(".").next().expect("could not find a \".\"");

    let i = Test { part: b };

    println!("{:#?}", i);
}

#[cfg(test)]
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
