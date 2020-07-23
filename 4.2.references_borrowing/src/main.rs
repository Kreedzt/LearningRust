fn main() {
    // 引用
    // let s1 = String::from("hello");
    
    // // let len = caculate_length(s1);

    // // 传递引用
    // let len = caculate_length(&s1);
    
    // println!("The length of '{}' is {}.", s1, len);


    // 可变引用
    // let mut s = String::from("hello");

    // change(&mut s);

    // Error: 特定作用域的特定数据: 仅能存在一个可变引用
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("s: {}", s);
    // println!("{}, {}", r1, r1);

    // 混合使用
    // let mut s = String::from("hello");

    // let r1 = &s; // OK
    // let r2 = &s; // OK
    // let r3 = &mut s; // Error

    // println!("{}, {} and {}", r1, r2, r3);
    // let r1 = &s;
    // let r2 = &s;
    // println!("{} and {}", r1, r2);

    // let r3 = &mut s; // OK
    // println!("{}", r3);

    let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回悬垂引用
//     // Error: 使用生命周期(第十章)
// }


// 正常返回
fn dangle() -> String {
    let s = String::from("hello");
    s
}

// fn caculate_length(s: String) -> usize {
//     s.len()
// }
// 拿到所有权, 源字符串失效

// 仅获取引用, 不获取所有权
fn caculate_length(s: &String) -> usize {
    s.len()
}


// 可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
