struct Cache<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    query: T,
    value: Option<E>
}

impl<T, E> Cache<T, E> 
where 
    T: Fn(E) -> E,
    E: Copy
{
    fn new(q: T) -> Cache<T, E> {
        Cache { 
            query: q, 
            value: None
        }
    }

    fn value(&mut self, v: E) -> E {
        match self.value {
            None => {
                let v = (self.query)(v);
                self.value = Some(v);
                v
            },
            Some(v) => v
        }
    }
}

fn main() {
    let x = vec![1, 2, 3];

    let y = |z| x.len() == z;

    let a = 2;

    y(a);
    y(a);

    println!("{:?} {}", x, a);
}
