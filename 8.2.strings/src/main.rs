fn main() {
    // let mut s = String::new();

    let data = "initial contents";

    // to_string 方法从字面量值创建 String
    let s = data.to_string();

    let s = "inital contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    // println!("s is {}", s);

    let mut s1 = String::from("foo");

    let s2 = "bar";

    s1.push_str(s2);

    // s2 依旧可以访问
    // println!("s2 is {}", s2);
    // println!("s1 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 类比函数签名:
    // fn add(self, s:&str) -> String
    // &String 强转为 &str
    // s1 失去所有权
    let s3 = s1 + &s2;

    // println!("s3: {}", s3);
    // Error:
    // println!("s1: {}", s1);
    // ^^ value borrowed here after move

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("s: {}", s);

    // format! 宏进行字符串连接, 并且不会影响所有权
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("s: {}", s);
    // println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    let len = String::from("Hola").len();

    println!("len: {}", len); // 3
    let len = String::from("Здравствуйте").len();
    println!("len: {}", len); // 24

    let hello = "Здравствуйте";
    // Error: 无法确定达到取第一个字符的效果
    // let answer = &hello[0];
    // ^^^^^^^^ string indices are ranges of `usize`

    // 字符串 slice
    let s = &hello[0..4];
    println!("s: {}", s);

    // Error: thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2034:5
    // let s = &hello[0..1];


    // 遍历字符串
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }


    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
