use crate::List::{Cons, Nil};

// error[E0072]: recursive type `List` has infinite size
// 该类型 "有无限的大小"
// enum List {
//     // = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable
//     Cons(i32, List),
//     Nil
// }

enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    // let b = Box::new(5);
    // 可以像数据储存在栈上的那样访问 box 中的数据.
    // 也会在离开作用域时释放
    // println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

    
}
