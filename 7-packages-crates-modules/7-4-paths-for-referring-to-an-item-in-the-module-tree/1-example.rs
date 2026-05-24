/*
chamando de duas formas diferentes add_to_waitlist função, na raiz do crate
*/

// arquivo
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


// hosting e add_to_waitlist são privados, não vai compilar