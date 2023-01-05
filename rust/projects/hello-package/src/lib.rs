mod back_of_house;
mod front_of_house;

use back_of_house::cook_order;
use front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    hosting::add_to_waitlist();
    
    cook_order();

    String::from("yummy yummy!")
}

/**
```
let a = 1;
let b = add_one(a);

assert_eq!(b, 2);
```
 */
pub fn add_one(a: i32) -> i32 {
    a + 1
}
