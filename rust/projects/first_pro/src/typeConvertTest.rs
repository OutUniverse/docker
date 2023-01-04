fn main() {
    use std::fmt::{self, Formatter};

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "(The point is ({}, {}))", self.x, self.y)
        }
    }

    let point = Point { x: 0, y: 0 };

    println!("{}", point.to_string());
}

#[cfg(test)]
fn test_fn_2_point() {
    fn foo() -> i32 {
        0
    }

    let point = foo as *const ();

    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(point)
    };

    assert_eq!(function(), 0);
}

#[cfg(test)]
fn test_impl_for_i32() {
    use std::fmt::Debug;

    trait Test {}

    fn foo<T: Test + Debug>(t: T) {
        println!("{:?}", t);
    }
    
    impl Test for &mut i32 {}

    let t: &mut i32 = &mut 0;

    foo(t);
}

#[cfg(test)]
fn test_foo_to_boo() {
    struct Foo {
        x: i32,
        y: i32,
    }
    
    #[derive(Debug)]
    struct Boo {
        a: i32,
        b: i32,
    }
    fn test_interpret(foo: Foo) -> Boo {
        let Foo { x, y } = foo;
        Boo {
            a: x,
            b: y,
        }
    }
    let foo = Foo { x: 1, y: 2 };

    let boo = test_interpret(foo);

    println!("{:?}", boo);
}


#[cfg(test)]
fn test_tryinto() {
    let a: u8 = 8;
    let b: u16 = 1500;
    let a_: u16 = a.try_into().unwrap();

    if a_ < b {
        println!("{}", a_);
    }

    let b_: u8 = match b.try_into() {
        Ok(b) => b,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        },
    };

    println!("{}", b_);
}

#[cfg(test)]
fn test_int_2_point() {
    println!("{}", i8::MAX);

    let mut a: [i32; 5] = [3, 5, 7, 1, 2];

    let p1 = a.as_mut_ptr();

    let first_point = p1 as usize;

    println!("{}, {}", first_point, p1 as i32);

    let second_point = first_point + 4;

    let p2 = second_point as *mut i32;

    unsafe {
        *p2 += 6;
    }

    let p3 = (second_point + 4) as *mut i32;

    unsafe {
        *p3 = 8;
    }

    println!("{:?}", a);
}
