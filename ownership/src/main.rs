fn main() {
    // let s = "hello";
    // {
    //     let b = "hello";
    //     println!("inner b: {}", b);
    // }
    
    // println!("outter s: {}", s);


    // 离开作用域自动释放
    // let s = String::from("hello");

    // println!("s: {}", s);

    // let mut s = String::from("hello");

    // s.push_str(", world!"); // 在字符串后面追加值
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // // 编译错误, 此时 s1 不再有效
    // println!("{}, world!", s1);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // 所有权与函数
    // let s = String::from("hello"); // s 进入作用域

    // takes_ownership(s); // s 值移动
    // // s 不再有效
    // // Error: value borrowed here after move
    // // println!("s: {}", s);
    
    // let x = 5;
    // makes_copy(x); // x移动到函数
    // // i32 是 Copy 的, 依旧有效
    // println!("x: {}", x);

    // 返回值与作用域
    // let s1 = gives_ownership();

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2); // s2 被移动到函数中
    // 结束时:
    // s3 移出作用域并被丢弃, s2 也移出作用域, 但已被移走
    // s1 移出作用域并被丢弃

    // 转移返回值的所有权
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 移除作用域, 调用 drop 释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
