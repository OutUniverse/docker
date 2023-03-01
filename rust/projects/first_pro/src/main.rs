use std::ops::Deref;

fn main() {
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
