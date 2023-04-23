

fn main() {
    
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
            println!("This is No.{} thread run, v is {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("This is No.{} main thread run", i);
        thread::sleep(Duration::from_millis(1));
    }
}
