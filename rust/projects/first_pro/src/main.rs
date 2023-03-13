struct TestDropOne;
impl Drop for TestDropOne {
    fn drop(&mut self) {
        println!("TestDropOne drop");
    }
}

struct TestDropTwo;
impl Drop for TestDropTwo {
    fn drop(&mut self) {
        println!("TestDropTwo drop");
    }
}

#[allow(dead_code)]
struct TestDropBox {
    one: TestDropOne,
    two: TestDropTwo
}
impl Drop for TestDropBox {
    fn drop(&mut self) {
        println!("TestDropBox drop");
    }
}

struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo drop");
    }
}

fn main() {
    let _x = TestDropBox {
        one: TestDropOne,
        two: TestDropTwo
    };

    let _foo = Foo;

    let a = 1;
    
    drop(a);
}
