// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

fn serve_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // pub 对枚举内所有成员都为公有
    pub enum Appetizer {
        Soup,
        Salad
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// use crate::front_of_house::hosting;
// 直接将函数引入作用域
use crate::front_of_house::hosting::add_to_waitlist;

// 重导出
pub use crate::front_of_house::hosting;

// use std::io;
// use std::io::Write;

// 合并语句
// use std::io::{self, Write};

// 通过 glob 运算符将所有的公有定义引入作用域
use std::collections::*;

pub fn eat_at_restaurant() {
    // 绝对地址
    // crate::front_of_house::hosting::add_to_waitlist();

    // 相对地址
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Error:
    // error[E0616]: field `seasonal_fruit` of struct `back_of_house::Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
