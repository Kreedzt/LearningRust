use std::ops::Add;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

// 当为 `Point` 实现 `Add` 时, 使用了默认的 `RHS`
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// 传递泛型
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*wabing arms furously*")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// 父 trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// error[E0277]: `Point` doesn't implement `std::fmt::Display`
// impl OutlinePrint for Point {}

// 满足 OutlinePrint 要求的限制
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// 孤儿模式 `禁止` 在 `Vec<T>` 上实现 Display
// 缺陷: Wrapper 是一个新类型, 没有定义于其值上的方法, 必须在此实现 `Vec<T>` 所有方法
// 如果希望拥有内部类型的每一个方法, 为封装类型实现 `Deref` trait 并返回其内部类型是一种解决方案
// newtype 模式
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Wrapper 是元组结构体, 0 索引 访问内部的 `Vec<T>`
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
    let person = Human;
    // 分别调用
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 直接调用了 `Dog` 之上的关联函数
    println!("A baby dog is called a {}", Dog::baby_name());

    // 关联函数调用, Rust 无法推断
    // error[E0283]: type annotations required: cannot resolve `_: Animal`
    // println!("A baby dog is called a {}", Animal::baby_name());

    // 完全限定语法: 表明希望调用的是 `Dog` 上 `Animal`
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // 父 trait
    let p = Point {
        x: 1,
        y: 3
    };
    p.outline_print();

    // newtype 模式
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
