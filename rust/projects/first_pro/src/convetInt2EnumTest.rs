fn main() {
    
}

fn use_transmute() {
    #[repr(i32)]
    #[allow(dead_code)]
    enum MyEnum {
        A,
        B,
        C
    }

    let a = MyEnum::B as i32;

    let b: MyEnum = unsafe {
        std::mem::transmute(a)
    };

    match b {
        MyEnum::A => println!("a"),
        MyEnum::B => println!("b"),
        MyEnum::C => println!("c"),
    };
}

#[cfg(test)]
fn use_try_from() {
    #[allow(dead_code)]
    enum MyEnum {
        A = 0,
        B,
        C,
    }

    impl TryFrom<i32> for MyEnum {
        type Error = ();

        fn try_from(x: i32) -> Result<Self, Self::Error> {
            match x {
                value if value == MyEnum::A as i32 => {
                    println!("{}", value);
                    Ok(MyEnum::A)
                },
                value if value == MyEnum::B as i32 => {
                    println!("{}", value);
                    Ok(MyEnum::B)
                },
                _ => Err(())
            }
        }
    }

    let x = MyEnum::B as i32;

    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        _ => println!("other"),
    };
}
