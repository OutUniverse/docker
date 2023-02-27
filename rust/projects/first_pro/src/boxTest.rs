fn main() {
    
}

#[cfg(test)]
fn change_string_2_static_str() {
    fn change() -> &'static str {
        Box::leak(String::from("test").into_boxed_str())
    }
}

#[cfg(test)]
fn dyn_test() {
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        name: &'static str,
        width: i32
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Button {} width is {}", self.name, self.width);
        }
    }

    struct Div {
        name: String,
        width: i32,
        height: i32
    }

    impl Div {
        fn new(name: String, width: i32, height: i32) -> Self {
            Self {name, width, height}
        }
    }

    impl Draw for Div {
        fn draw(&self) {
            println!("Div draw");
        }
    }

    fn test(t: Box<dyn Draw>) {
        t.draw();
    }

    test(Box::new(Button {name: "Btn", width: 12}));
    test(Box::new(Div::new(String::from("Div"), 12, 13)));
}

#[cfg(test)]
fn convert_dst_2_sized() {
    // Error, because Test is DST
    // enum Test {
    //     Cons(i32, Test),
    //     Nil
    // }

    enum Test {
        Cons(i32, Box<Test>),
        Nil
    }
}

#[cfg(test)]
fn test_avoid_create_new_data_on_stack() {
    let a = [1; 1000];

    let b = a;

    // arr impl Copy, so b will create a new array
    println!("{:?} {:?}", a, b);

    let c = Box::new([2;1000]);

    let d = c;


    // c move to d
    // println!("{:?}", c);
    println!("{:?}", d);
}

#[cfg(test)]
fn test_create_data_on_stack() {
    let a = Box::new(1);

    println!("{:?}", a);
}
