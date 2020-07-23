fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s); // 5

    // s.clear(); // 清空字符串, 使其 = ""

    // word 在此时的值仍然是 5, 但是现在word的值完全无效
    // println!("word: {}", word);

    // slice
    // let s = String::from("hello world");

    // 左闭右开
    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{}-{}", hello, world);

    // let len = s.len();
    // 3 直到末尾 
    // let slice = &s[3..len];
    // println!("slice: {}", slice);
    // 3直到末尾
    // let slice = &s[3..];
    // println!("slice: {}", slice);

    // 整个字符串截取
    // let slice = &s[0..len];
    // println!("slice: {}", slice);
    // 整个字符串截取
    // let slice = &s[..];
    // println!("slice: {}", slice);
    
    // let mut s = String::from("hello world");
    // 返回值 &str 为不可变引用
    // let word = first_word(&s);

    // 尝试清除
    // s.clear(); // Error: mutable borrow occurs here
    // println!("The first word is: {}", word);

    let my_string = String::from("hello world");

    // 传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // 传入字面量字面值的 slice 
    let word = first_word(&my_string_literal[..]);

    // 字符串字面值就是 字符串 `slice`, 故可以直接传递
    let word = first_word(my_string_literal);

    // 其他类型的 slice
    // 类型: &[i32]. 跟其他字符串 slice 的工作方式一样, 可以对其他所有集合使用这类 slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

// 改为 slice 参数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 返回第一个单词
// &str: 指向二进制程序特定位置的 `slice`
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         } 
//     }

//     &s[..]
// }

// 查找单词结尾, 返回一个字节索引值
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             // 找到返回索引
//             return i;
//         }
//     }

//     // 否则返回长度
//     s.len()
// }
