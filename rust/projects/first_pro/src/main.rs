

fn main() {
    use std::thread;
    use std::cell::RefCell;
    use std::thread::LocalKey;

    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }

    struct Test {
        foo: &'static LocalKey<RefCell<usize>>,
    }

    impl Test {
        fn new() -> Self {
            Test {
                foo: &FOO
            }
        }
    }

    let handle = thread::spawn(|| {
        let b = Test::new();

        b.foo.with(|f| {
            *f.borrow_mut() = 2;
            println!("{}", *f.borrow());
            *f.borrow_mut() = 1;
            println!("{}", *f.borrow());
        });
    });

    handle.join().unwrap();

    let a = Test::new();

    a.foo.with(|f| {
        println!("{}", *f.borrow());
    });
}

#[cfg(test)]
fn test_thread_local_in_struct() {
    use std::cell::RefCell;

    struct Stest;

    impl Stest {
        thread_local! {
            static S_LOCAL: RefCell<usize> = RefCell::new(0);
        }
    }

    Stest::S_LOCAL.with(|f| {
        print!("{}", *f.borrow());
    });
}

#[cfg(test)]
fn test_thread_local() {
    use std::cell::RefCell;
    use std::thread;

    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let spawned_thread = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
            println!("{}", *f.borrow());
        });
    });

    spawned_thread.join().unwrap();

    FOO.with(|f| {
        println!("{}", *f.borrow());
    });
}

#[cfg(test)]
fn test_barrier() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for i in 0..6 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} started", i);
            b.wait();
            println!("Thread {} finished", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
fn test_grand_child_thread_stop() {
    use std::thread;
    use std::time::Duration;

    let threads = thread::spawn(|| {
        println!("I'm child thread run");
        thread::spawn(|| {
            loop {
                println!("I'm grandChild thread run");
            }
        });
    });

    threads.join().unwrap();

    println!("Child is finished");

    thread::sleep(Duration::from_millis(1));
}

#[cfg(test)]
fn test_spawn() {
    use std::thread;
    use std::time::Duration;

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("This is Number {} on on spawned thread, v is {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("This is Number {} on main thread run", i);
        thread::sleep(Duration::from_millis(1));
    }
}
