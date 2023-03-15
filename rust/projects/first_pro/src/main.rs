fn main() {
    use std::cell::RefCell;

    let s = RefCell::new(String::from("test"));

    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{} {}", s1, s2);
}

#[cfg(test)]
fn test_cell() {
    use std::cell::Cell;

    let c = Cell::new("aaa");

    let one = c.get();

    c.set("bbb");

    let two = c.get();

    println!("{} {}", one, two);
}
