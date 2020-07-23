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

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }


    // 使用 if let
    if let Some(0u8) = some_u8_value {
        println!("0u8");
    }

    let coin = Coin::Quarter(UsState::Alabama);

    // let mut count = 0;

    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1
    // }

    let mut count = 0;
    // 使用 if let
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
