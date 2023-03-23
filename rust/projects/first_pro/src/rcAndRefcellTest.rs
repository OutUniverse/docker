fn main() {
    use std::rc::Rc;
    use std::cell::Cell;

    let a = Rc::new(Cell::new("hello"));
    let b = a.clone();
    let c = a.clone();

    c.set("bbb");

    println!("{:?} {:?} {:?}", a, b, c);
}

#[cfg(test)]
fn test_mut_vec_to_cell_arr() {
    use std::cell::Cell;
    fn is_even(i: i32) -> bool {
        i % 2 == 0
    }

    fn retain_even(nums: &mut Vec<i32>) {
        let mut i = 0;
        let slice = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
        for num in slice.iter().filter(|&num| is_even(num.get())) {
            slice[i].set(num.get());
            i += 1;
        }
        nums.truncate(i);

        println!("{:?} {}", nums, i);
    }

    retain_even(&mut vec![1, 2, 3, 4]);
}

#[cfg(test)]
fn test_rc_combine_refcell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let s = Rc::new(RefCell::new("Test".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();

    s1.borrow_mut().push_str(" test");

    println!("{:?} {:?} {:?}", s, s1, s2);
}

#[cfg(test)]
fn test_relcell_impl() {
    use std::cell::RefCell;

    pub trait Messenger {
        fn send(&self, msg: String);
    }

    struct MsgQueue {
        msg_cache: RefCell<Vec<String>>
    }

    impl Messenger for MsgQueue {
        fn send(&self, msg: String) {
            self.msg_cache.borrow_mut().push(msg);
        }
    }
}

#[cfg(test)]
fn test_refcell() {
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
