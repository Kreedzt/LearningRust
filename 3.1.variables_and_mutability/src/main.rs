fn main() {
    // let mut x = 5;
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // let spaces = "  ";
    // let spaces = spaces.len(); // OK

    let mut spaces = "  ";
    spaces = spaces.len(); // Error: 不能改变变量类型
}
