#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // 使用元组
    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // 使用结构体
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // 通过派生 trait
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // println!("rect1 is {}", rect1);

    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
    // println!("rect1 is {:?}", rect1);

    // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {:?}", rect1);

    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
