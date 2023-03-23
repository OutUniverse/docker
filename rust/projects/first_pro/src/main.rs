use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;

struct Gadgets {
    name: String,
    owner: RefCell<Rc<Owner>>
}

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Rc<Gadgets>>>
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
            owner: RefCell::new(Rc::clone(&owner_one))
        }
    );

    owner_one.gadgets.borrow_mut().push(Rc::clone(&gadget_one));

    
}
