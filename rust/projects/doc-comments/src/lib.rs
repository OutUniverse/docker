//! # 文档注释
//! 
//! 该库用于文档注释的教学

// in lib.rs
pub mod compute;

/// Add one to the given value and return the value
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = doc_comments::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** Add two to the given value and return a new value

Examples
```
let arg = 5;
let answer = doc_comments::add_two(arg);

assert_eq!(7, answer);
```
*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}
