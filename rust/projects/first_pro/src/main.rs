use crate::List::{Cons, Nil};
use std::{rc::Rc, fmt::Debug};
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}


#[allow(dead_code)]
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

// impl Debug for List {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Cons(i, _) => write!(f, "Cons {{ {} }}", i),
//             Nil => write!(f, "Nil")
//         }
//     }
// }
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a ref count is {}", Rc::strong_count(&a));
    println!("b ref count is {}", Rc::strong_count(&b));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count is {}", Rc::strong_count(&b));

    println!("{:?}", a.tail());
}
