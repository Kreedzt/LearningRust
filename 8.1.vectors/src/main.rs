fn main() {
    // 提供类型注解, 明确内部值类型
    // let v: Vec<i32> = Vec::new();

    // vec! 宏可以方便创建一个新的 Vec

    // 以下代码类型默认为 Vec<i32>

    // let v = vec![1, 2, 3];

    // println!("123");

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    {
        let v = vec![1, 2, 3, 4];
    } // 离开作用域自动丢弃

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/libcore/slice/mod.rs:2695:10
    // panic 崩溃
    // let does_not_exist = &v[100];

    // 返回 None 而不是崩溃
    let does_not_exist = v.get(100);
    

    // 尝试引用后新增元素
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);
    //    |     ^^^^^^^^^ mutable borrow occurs here

    // println!("The first element is: {}", first);

    // 遍历

    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    // 修改指向的值
    for i in &mut v {
        *i += 50;
    }
    
    // 使用枚举来存储多种类型

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
