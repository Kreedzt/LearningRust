// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25
//     }
// }

// 绑定值模式
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // 匹配是穷尽的, 注释以下代码导致编译错误:
        //    |           ^ pattern `None` not covered
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    // println!("value_in_cents(Penny): {}", value_in_cents(Coin::Penny));
    // println!("value_in_cents(Penny): {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // 通配符
        _ => (),
        // error: expected expression, found reserved identifier `_`
        // _ => println!("{}", _),
    }
}
