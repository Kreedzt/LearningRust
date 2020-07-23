// 具有返回值的函数
fn five() -> i32 {
    5
}

fn main() {
    let y = 6; // 语句
    // another_function(5, 6);

    // let x = (let y = 6); // Error: 语句不返回值

    let x = 5;

    // {} 也是表达式
    let y = {
        let x = 3;
        x + 1 // 返回表达式
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}


fn another_function(x: i32, y: i32) {
    // println!("Another function.");
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}


fn plus_one(x: i32) -> i32 {
    // 加分号会导致错误, 识别成语句
    x + 1
}
