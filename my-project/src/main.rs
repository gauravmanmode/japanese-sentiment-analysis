mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("adding to waitlist");
        }

        fn seat_at_table() {}
    }
}

pub fn eat_at_restaurent() {
    front_of_house::hosting::add_to_waitlist();
}
