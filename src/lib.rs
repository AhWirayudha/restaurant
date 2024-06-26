mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// use crate::front_of_house::hosting;
// we can use pub, so external scope can use it
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();
    // using use
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// cannot use `use` in different scope
// mod customer { // different scope
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// sample rightway using use with same name in standard library, Result
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// then u can use as to change the name
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// use external package, need to install dependencie with cargo except std library
use rand::Rng; // external library in crates.io
use std::collections::HashMap; // std library

fn function3() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// using nested use
// --snip--
use std::{cmp::Ordering, io};
// --snip--

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// glob *, bring all public item defined
use std::collections::*;
