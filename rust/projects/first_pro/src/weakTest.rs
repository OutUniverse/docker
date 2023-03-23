// use crate::TestTwo::One;

use crate::List::{Cons, Nil};
// use std::ops::Deref;
use std::{rc::Rc, fmt::Debug};
use std::cell::RefCell;

// #[derive(Debug)]
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

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(i, _) => write!(f, "Cons {{ {} }}", i),
            Nil => write!(f, "Nil")
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("{:?} List dropped", self);
    }
}

// #[allow(dead_code)]
// fn judge<T>(a: Box<dyn Deref<Target = T>>) {
//     println!("impl Deref for");
// }

    // struct Test;

    // impl Drop for Test {
    //     fn drop(&mut self) {
    //         println!("Struct Test Drop");
    //     }
    // }

    // enum TestTwo {
    //     One(Rc<Test>)
    // }

fn main() {
    // let test = One(Rc::new(Test));

    // drop(test);

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a ref count is {}", Rc::strong_count(&a));
    println!("b ref count is {}", Rc::strong_count(&b));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count is {}", Rc::strong_count(&b));

    println!("{:?}", a.tail());

    let c = Rc::new(String::from("test"));
    // judge(Box::new(c));
    // Test deref for Rc
    // let e: &String = &c;
    println!("{:?}", c);

    let g: RefCell<Vec<u32>> = RefCell::new(vec![1, 2, 3]);
    g.borrow_mut().push(1);

    let d = Rc::new(5);
    let e = Rc::new(6);

    let f = *d + *e;

    println!("{:?}", f);

    let five = Rc::new(String::from("weak test"));

    let weak_five = Rc::downgrade(&five);

    let strong_five = weak_five.upgrade();

    if let Some(item) = strong_five {
        println!("{:?}", item);
    }

    drop(five);

    let strong_five = weak_five.upgrade();

    if let None = strong_five {
        println!("None value");
    }

}
