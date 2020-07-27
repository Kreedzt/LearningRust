use std::io;
use std::io::{ErrorKind, Read};
use std::fs::File;
use std::error::Error;

// 使用 Rusult类型来处理潜在的错误: 系统自带枚举:
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// 修改返回值以使用 ?
// Box<dyn Error> 称为 trait object, 目前理解为允许 "任何类型的错误"
fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");
    // 打开不存在的文件
    // 写错误类型以编译器提示
    // let f:u32 = File::open("hello.txt");
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     // thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:19:13
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };

    // 匹配不同的错误
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error)
    //     }
    // };

    // 使用闭包(十三章详解)
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // unwrap 和 expect
    // unwrap 返回 Ok 的值, 否则调用 panic!
    // let f = File::open("hello.txt").unwrap();
    
    // expect 传入错误信息作为参数, 而不是默认的 panic!
    // thread 'main' panicked at 'Failed to open hello.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:999:5
    // 错误信息指向文本的开始, 将会更容易找到代码中的错误信息来自何处
    // 如果在多出使用 unwrap, 所有unwrap调用都打印相同的信息, 很难分析
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // 尝试在 main 函数中使用 ?
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
    let f = File::open("hello.txt")?;
    Ok(())
}

// 传播, ?, 链式调用
fn read_username_from_file() -> Result<String, io::Error> {
    // 利用 ? 链式调用
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}

// 传播, 简写: ?
// fn read_username_from_file() -> Result<String, io::Error>  {
//     // ? 运算符把 Ok的值返回, Err将立即return
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// 传播, Ok: String, Err: io::Error
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
    
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e)
//     }
// }
