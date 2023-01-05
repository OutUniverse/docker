mod back_of_house;
mod front_of_house;

use back_of_house::cook_order;
use front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    hosting::add_to_waitlist();
    
    cook_order();

    String::from("yummy yummy!")
}
