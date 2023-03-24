use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Gadgets {
    name: String,
    owner: RefCell<Weak<Owner>>
}

impl Drop for Gadgets {
    fn drop(&mut self) {
        println!("{:?} is dropped", self.name);
    }
}

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Rc<Gadgets>>>
}

impl Drop for Owner {
    fn drop(&mut self) {
        println!("{:?} is dropped", self.name);
    }
}

fn main() {
    let owner_one = Rc::new(
        Owner {
            name: "Owner One".to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    );

    let gadget_one = Rc::new(
        Gadgets {
            name: "Gadgets One".to_string(),
            owner: RefCell::new(Rc::downgrade(&owner_one))
        }
    );

    let gadget_two = Rc::new(
        Gadgets {
            name: "Gadgets Two".to_string(),
            owner: RefCell::new(Rc::downgrade(&owner_one))
        }
    );

    owner_one.gadgets.borrow_mut().push(Rc::clone(&gadget_one));
    owner_one.gadgets.borrow_mut().push(Rc::clone(&gadget_two));

    for gadget in owner_one.gadgets.borrow().iter() {
        match gadget.owner.borrow().upgrade() {
            Some(item) => println!("{:?} is owned by {:?}", gadget.name, item.name),
            _ => println!("no owner")
        }
    }
}
