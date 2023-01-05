pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Add to waitlist");
    }

    pub fn seat_at_table() {
        println!("Seat at table");
    }
}

mod serving {
    fn take_order() {
        println!("Take order");
    }

    fn serve_order() {
        println!("Serve order");
    }

    fn take_payment() {
        println!("Take payment");
    }
}
