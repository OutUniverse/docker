use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&Rc<List>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

struct A {
    a: String
}

impl A {
    fn test(&self) -> &String {
        &self.a
    }
}

fn main() {
    
}
