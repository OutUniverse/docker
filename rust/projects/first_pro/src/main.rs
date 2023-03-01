fn main() {
    let s = String::from("test");

    let s1: &str = &s;
    let s2: String = s.to_string();

    println!("{} {:?}", s1, s2);
}

#[cfg(test)]
fn test_continuous_deref() {
    let s = Box::new(String::from("test"));

    fn test(a: &str) {
        println!("{}", a);
    }

    test(&s);
}

#[cfg(test)]
fn test_string_deref() {
    let s = String::from("hello world");

    fn test(a: &str) {
        print!("{}", a);
    }

    test(&s);
}

#[cfg(test)]
fn test_my_box() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    trait Test {
        type Test;

        fn test(self) -> Self::Test;
    }

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Test for MyBox<T> {
        type Test = T;

        fn test(self) -> Self::Test {
            self.0
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let a = MyBox::new(1);

    assert_eq!(1, *a);
}
