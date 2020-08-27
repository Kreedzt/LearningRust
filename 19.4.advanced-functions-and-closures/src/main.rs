fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // 对 map 使用闭包
    // let list_of_strings: Vec<String> = list_of_numbers
    //     .iter()
    //     .map(|i| i.to_string())
    //     .collect();

    // 传递函数代替闭包
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        // 此处使用完全限定语法, 因为存在多个 to_string 函数
        .map(ToString::to_string)
        .collect();

    let list_of_status: Vec<Status> =
        // 初始化语法
        (0u32..20)
        // 实现了闭包 trait 的函数指针
        .map(Status::Value)
        .collect();
}

// 无法直接返回闭包
// error[E0277]: the size for values of type `(dyn std::ops::Fn(i32) -> i32 + 'static)` cannot be known at compilation time
// Rust 不知道需要多少空间来储存闭包
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// 使用 trait 对象
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
