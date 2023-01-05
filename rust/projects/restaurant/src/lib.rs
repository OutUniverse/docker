mod front_restaurant;

pub use crate::front_restaurant::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    hosting::seat_at_table();
}

fn main() {
    eat_at_restaurant();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
