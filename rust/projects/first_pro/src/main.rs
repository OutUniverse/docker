trait Birds {
    fn fly(&self);
}

trait Swan {
    fn fly(&self);
}

struct Person;

impl Birds for Person {
    fn fly(&self) {
        println!("Birds fly");
    }
}

impl Swan for Person {
    fn fly(&self) {
        println!("Swan fly");
    }
}

impl Person {
    fn fly(&self) {
        println!("Person fly");
    }
}

struct Humen;

impl Humen {
    fn new() -> String {
        String::from("Humen")
    }
}

trait Test {
    fn new() -> String;
}

impl Test for Humen {
    fn new() -> String {
        String::from("Test")
    }
}

struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, a: Self::A, b: Self::B) -> bool;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: Self::A, b: Self::B) -> bool {
        self.0 == a && self.1 == b
    }
}

fn main() {
    let person = Person;

    person.fly();
    Birds::fly(&person);
    Swan::fly(&person);

    println!("{:?}", Humen::new());
    println!("{:?}", <Humen as Test>::new());

    let container = Container(1, 2);

    println!("{}", container.contains(1, 2));
}