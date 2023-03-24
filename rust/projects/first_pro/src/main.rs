use std::rc::{Weak, Rc};
use std::cell::RefCell;

struct Node {
    value: u32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Node {} is dropped", self.value);
    }
}

use crate::Test::{One, Nil};

#[derive(Debug)]
enum Test {
    One(u32, A),
    Nil
}

#[derive(Debug)]
struct A {
    next: RefCell<Rc<Test>>
}

fn main() {
    let a = Rc::new(One(1, A {next: RefCell::new(Rc::new(Nil))}));
    // match a {
    //     One(value, _) => println!("{}", value),
    //     _ => println!("none"),
    // }

    let b = One(2, A {next: RefCell::new(Rc::new(Nil))});

    match b {
        One(value, _) => println!("{}", value),
        _ => println!("none"),
    }
    // let b = Rc::new(One(2, A {next: RefCell::new(Rc::clone(&a))}));
    
    // println!("{:?}", a.0);


    let node_one = Rc::new(
        Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(Vec::new())
        }
    );

    let node_two = Rc::new(
        Node {
            value: 2,
            parent: RefCell::new(Rc::downgrade(&node_one)),
            child: RefCell::new(Vec::new())
        }
    );

    node_one.child.borrow_mut().push(Rc::clone(&node_two));

    {
        let node_three = Rc::new(
            Node {
                value: 3,
                parent: RefCell::new(Rc::downgrade(&node_one)),
                child: RefCell::new(Vec::new())
            }
        );

        node_one.child.borrow_mut().push(Rc::clone(&node_three));

        for node in node_one.child.borrow().iter() {
            if let Some(item) = node.parent.borrow().upgrade() {
                println!("Node {} is the child for Node {}", node.value, item.value);   
            }
        }
    }

    for node in node_one.child.borrow().iter() {
        if let Some(item) = node.parent.borrow().upgrade() {
            println!("Node {} is the child for Node {}", node.value, item.value);   
        }
    }
}
