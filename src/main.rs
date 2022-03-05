mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> i32 {
            3
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn main() {
    let x = self::front_of_house::hosting::add_to_waitlist();
    println!("{}", x);
}
