fn main() {
    use std::rc::Rc;

    let a = Rc::new(String::from("test"));

    println!("{}", Rc::strong_count(&a));

    let b = Rc::clone(&a);

    println!("{}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&b);

        println!("{}", Rc::strong_count(&c));
    }

    println!("{}", Rc::strong_count(&a));
}
