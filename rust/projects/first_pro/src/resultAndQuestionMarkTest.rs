use std::fs::{File, self};
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    let a = [1, 2, 3];

    let a = a.map(|item| item + 1);

    println!("{:?}", a);

    test_alias();
}

fn test_alias() {
    type Res<T> = Result<T, ParseIntError>;

    fn test_multiple(str1: &str, str2: &str) -> Res<i32>{
        str1.parse::<i32>().and_then(|num1| 
            {str2.parse::<i32>().and_then(|num2| Ok(num1 * num2))})
    }

    fn print(rel: Res<i32>) {
        match rel {
            Ok(i) => println!("value is {}", i),
            Err(e) => println!("error: {}", e)
        }
    }

    print(test_multiple("1", "2"));
    print(test_multiple(",", "2"));
}

#[cfg(test)]
fn test_and_then() -> Result<i32, ParseIntError> {
    let c = "3";

    c.parse::<i32>().and_then(|num| Ok(num + 2))
}

#[cfg(test)]
fn test_map() -> Result<i32, ParseIntError> {
    let b = "2";

    b.parse::<i32>().map(|num| num + 2)
}

#[cfg(test)]
fn test_option_direct(a: &[i32]) -> Option<&i32> {
    a.get(0)
}

#[cfg(test)]
fn test_option() -> Option<i32> {
    let a = [1, 2, 3, 4];

    let v = a.get(0)?;

    Some(*v)
}

#[cfg(test)]
fn test_fs_read_to_string() -> Result<String, io::Error> {
    fs::read_to_string("test.txt")
}

#[cfg(test)]
fn use_question_mark() -> Result<String, Box<dyn std::error::Error>> {
    let mut s = String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
fn traditional_return_value() -> Result<String, io::Error> {
    let f = File::open("test.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => match File::create("test.txt") {
                Ok(_) => {
                    match fs::write("test.txt", "asdasdas") {
                        Ok(_) => match File::open("test.txt") {
                            Ok(f) => f,
                            Err(e) => return Err(e),
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e),
            },
            _ => return Err(e),
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
fn use_unwrap() {
    File::open("test1.txt").unwrap();
}

#[cfg(test)]
fn use_expect() {
    File::open("test2.txt").expect("test2.txt not found");
}
