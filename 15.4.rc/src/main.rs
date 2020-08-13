// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

enum List {
    // 共享所有权, 自增引用计数
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5,
    //              Box::new(Cons(10,
    //                            Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // error[E0382]: use of moved value: `a`
    // let c = Cons(4, Box::new(a));
    // 可以改变 `Cons` 定义来存放一个引用, 不过必须指定生命周期函数


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // 使用 `Rc::clone(&a)`, 而不是 `a.clone`, 后者为深拷贝, 前者仅增加计数
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    // 离开作用域自动清理计数
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
