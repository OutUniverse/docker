#[allow(dead_code)]
fn main() {
    use std::rc::Rc;

    struct Owner {
        name: String
    }

    struct Gadget {
        name: String,
        owner: Rc<Owner>
    }

    let owner_one = Rc::new(Owner { name: "owner one".to_string() });

    let gadget_one = Gadget { name: "gadgetone".to_string(), owner: Rc::clone(&owner_one) };
    let gadget_two = Gadget { name: "gadgettwo".to_string(), owner: Rc::clone(&owner_one) };

    println!("All owner rc count {}", Rc::strong_count(&owner_one));

    drop(owner_one);

    println!("All owner rc count {}", Rc::strong_count(&gadget_one.owner));

    drop(gadget_one);

    println!("All owner rc count {}, owner name is {:?}", Rc::strong_count(&gadget_two.owner), gadget_two.owner.name);

    println!("owner name is {:?}", gadget_two.owner.name);

    fn test_rc_owner (_a: Rc<Owner>) {

    }

    test_rc_owner(gadget_two.owner);
}

#[cfg(test)]
fn test_rc() {
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
