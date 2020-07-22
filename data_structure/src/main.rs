fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // OK
    // let guess = "42".parse().expect("Not a number"); // Error: 编译器不总是能推断类型
    let x = 2.0; // f64 双精度
    let y: f32 = 3.0; // f32 单精度

    let sum = 5 + 10;
    
    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;
    
    let c = 'z'; // char 为 4 字节, unicode标量值
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // 元祖: 混合数据类型, 声明后长度不可变

    let tmp = (500, 6.4, 1);

    let (x, y, z) = tup; // 解构
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // 通过下标访问
    
    let six_point_four = x.1;
    
    let one = x.2;

    let a = [1, 2, 3, 4, 5]; //数组, 栈上分配内存
    // Vector 允许增长或减少, 数组不允许

    // [类型; 数量]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // [值, 数量]
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5]; // 栈上分配内存, 可以用索引访问元素
    let first = a[0];
    let second = a[1];

    let index = 10;

    let element = a[index]; // 越界访问, 抛出运行时错误

    println!("The value of element is: {}", element);
}
