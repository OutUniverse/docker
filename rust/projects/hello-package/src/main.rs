mod front_of_house;

use front_of_house::hosting::*;
use hello_package::*;

fn main() {
    assert_eq!(seat_at_table(), "sit down please");
    assert_eq!(eat_at_restaurant(),"yummy yummy!");
}
