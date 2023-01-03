use std::convert::TryInto;

fn main() {
    let a: u8 = 8;
    let b: u16 = 1500;

    let b_: u8 = b.try_into().unwrap();

    if a < b_ {
        println!("{}", b_);
    }
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
