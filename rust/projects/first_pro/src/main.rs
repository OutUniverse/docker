#[allow(dead_code)]
struct Cache<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    query: T,
    value: Option<E>
}

#[allow(dead_code)]
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
    let mut x = vec![1, 2, 3];

    let y = |z| z > x.len();
    y(4);
}

#[cfg(test)]
fn fn_once<F>(f: F)
where
    F: FnOnce(usize) -> bool
{
    println!("{}", f(4));
}

#[cfg(test)]
fn test_fn_mut() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello");
    }

    exec(update_string);

    println!("{:?}", s);
}

#[cfg(test)]
fn test_mut_fuc_var() {
    let mut s = String::new();

    let mut update_string = |a| s.push_str(a);

    update_string("hello");

    println!("{:?}", s);
}

#[cfg(test)]
fn test_move_var() {
    let x = vec![1, 2, 3];

    let y = move |z| x.len() == z;

    let a = 2;

    y(a);
    y(a);

    // println!("{:?} {}", x, a);
}

#[cfg(test)]
fn fn_test<F>(f: F)
where
    F: FnOnce(usize) -> bool + Copy
{   
    f(1);
    f(2);
    f(3);
}
