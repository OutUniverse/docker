#[derive(Debug)]
#[allow(dead_code)]
struct Test<'a> {
    part: &'a str,
}

impl<'a> Test<'a> {
    fn test_lifetime_b<'b>(&'a self, b: &'b str) -> &'b str
    where 
        'a: 'b,
    {
        if self.part.len() >= b.len() {
            self.part
        } else {
            b
        }
    }
}

#[allow(dead_code)]
fn main() {
    let a = String::from("This is a test. You must believe");
    let b = a.split(".").next().expect("could not find a \".\"");

    let i = Test { part: b };

    println!("{:?}", i.test_lifetime_b("lalala, lalala, woshimaibaodexiaohuajia."));
}

#[cfg(test)]
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
