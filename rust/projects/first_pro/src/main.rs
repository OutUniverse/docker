fn main() {
    
}

#[cfg(test)]
fn test_mut_ptr() {
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        reff: *mut String
    }

    let mut a = SelfRef{
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
