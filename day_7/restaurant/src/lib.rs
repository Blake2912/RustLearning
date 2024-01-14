mod front_of_hose {

    fn do_something() {}

    mod hosting {
        fn add_to_waitlist() {
            crate::front_of_hose::serving::take_order()
        }

        fn seat_at_table() {
            super::do_something()
        }
    }

    mod serving {
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

/*
Module Structure
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/