//! hello-package包是库包
mod back_of_house;
mod front_of_house;

use back_of_house::cook_order;
use front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    hosting::add_to_waitlist();
    
    cook_order();

    String::from("yummy yummy!")
}

/// `add_one` 将指定值加1，返回一个[`Option`]类型
/// 跳转到[`crate::division`]
/// # Examples
/// 
///  ```
/// let a = 1;
/// let b = hello_package::add_one(a);
/// 
/// assert_eq!(b, 2);
/// ```
pub fn add_one(a: i32) -> Option<i32> {
    Some(a + 1)
}

/**
 * # Panics
 * 
 * The funtion panics if the second argument is 0
 * 
 * ```rust,should_panic
 * // panics on division by zero
 * hello_package::division(10, 0);
 * ```
 */

 pub fn division(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }

    a / b
 }

 /// ```rust
 /// # fn try_main() -> Result<(), String> {
 ///    let res = hello_package::try_div(10, 0)?;
 /// # Ok(())
 /// # }
 /// # fn main() {
 /// #   try_main().unwrap();
 /// # }
 /// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    }

    Ok(a / b)
}

#[doc(alias= "X")]
#[doc(alias = "yz")]
pub struct Xyz;

#[doc(alias= "A")]
#[doc(alias = "bc")]
pub struct Abc;
