// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
// 
//         fn seat_at_table() {}
//     }
// 
//     mod serving {
//         fn take_order() {}
// 
//         fn serve_order() {}
// 
//         fn take_payment() {}
//     }
// }
// 
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
// 
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
// 
// fn main() {}

// fn serve_order() {}
// 
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }
// 
//     fn cook_order() {}
// }
// 
// fn main() {}
//

// #![allow(unused_variables)]
// fn main() {
//     mod back_of_house {
//         pub struct Breakfast {
//             pub toast: String,
//             seasional_fruit: String,
//         }
// 
//         impl Breakfast {
//             pub fn summer(toast: &str) -> Breakfast {
//                 Breakfast {
//                     toast: String::from(toast),
//                     seasional_fruit: String::from("peaches"),
//                 }
//             }
//         }
//     }
// }
// 
// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change my mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// 
//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }

#![allow(unused_variables)]
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
// 
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
// 
// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// 
//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }
// 
// fn main() {
//     eat_at_restaurant();
// }
//

// #![allow(unused_variables)]
// fn main() {
//     mod back_of_house {
//         pub enum Appetizer {
//             Soup,
//             Salad,
//         }
//     }
// 
//     fn eat_at_restaurant() {
//         let order1 = back_of_house::Appetizer::Soup;
//         let order2 = back_of_house::Appetizer::Salad;
//     }
// 
//     eat_at_restaurant();
// }
// 

// `use` keyword

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 
// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// 
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
// 
// fn main() {}

// use std::collections::HashMap;
// 
// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }
// 


// Re-exporting Names with pub use

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 
// pub use crate::front_of_house::hosting;
// 
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
// 
// fn main() {}
//

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
} 
