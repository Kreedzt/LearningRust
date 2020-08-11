use std::ops::Deref;

// 自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref trait
impl<T> Deref for MyBox<T> {
    // 定义用于此 trait 的关联类型
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // 追踪指针值
    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // 像引用一样使用 Box<T>
    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
    // assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // 解引用强制多态:
    // &MyBox<String> -> &String(再次 Deref) -> &str
    hello(&m);

    // 非强制多态使用
    // (*m): MyBox<String> -> String
    // 
    hello(&(*m)[..]);
}
