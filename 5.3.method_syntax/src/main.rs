#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// 关联函数, :: 调用
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

// 多 impl 块
impl Rectangle {
    // 类似 &Reactangle, 不可变地借用 self, 可变可传递 &mut self
    // 警告: 使用 self 通常在转换成别的实例
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(200);
    println!("square1: {:#?}", square1);
}
