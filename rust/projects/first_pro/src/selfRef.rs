fn main() {
    
}

#[cfg(test)]
fn test_pin() {
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::ptr::NonNull;

    struct Unmoved {
        data: String,
        slice: NonNull<String>,
        _pin: PhantomPinned
    }

    impl Unmoved {
        fn new(data: String) -> Pin<Box<Unmoved>> {
            let res = Unmoved { 
                data,
                slice: NonNull::dangling(),
                _pin: PhantomPinned
            };

            let mut boxed = Box::pin(res);

            let slice = NonNull::from(&boxed.data);

            unsafe {
                let mut_ref = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).slice = slice;
            }

            boxed
        }
    }

    let unmoved = Unmoved::new("lalal".to_string());


    assert_eq!(unmoved.slice, NonNull::from(&unmoved.data));
}

#[cfg(test)]
fn test_mut_ptr() {
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        reff: *mut String
    }

    let mut a = SelfRef {
        value: "a".to_string(),
        reff: std::ptr::null_mut()
    };

    let mut b = "b".to_string();

    a.reff = &mut a.value;

    a.reff = &mut b;

    unsafe {
        (&mut *a.reff).push_str("b");
        println!("{:?}", *a.reff);
    }
}

#[cfg(test)]
fn test_const_ptr() {
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        reff: *const String
    }

    let mut a = SelfRef {
        value: "he".to_string(),
        reff: std::ptr::null(),
    };

    a.reff = &a.value;

    unsafe {
        println!("{:?}", *a.reff);
    }
}
