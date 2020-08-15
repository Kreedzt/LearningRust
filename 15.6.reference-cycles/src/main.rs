use std::rc::{Rc, Weak};
use std::cell::RefCell;
// use crate::List::{Cons, Nil};

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None
//         }
//     }
// }

// 创建树形数据结构
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>
// }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // // 创建一个引用循环
    // // a -> 5, _ -> b -> 10,_ -> a
    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // thread 'main' has overflowed its stack
    // println!("a next item = {:?}", a.tail());

    // 创建树形数据结构

    // 此处 leaf 无法知道父级
    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![])
    // });

    // let branch = Rc::new(Node {
    //     value: 5,
    //     // `leaf` 中的 Node 现在有 2 个所有者: `leaf` 和 `branch`
    //     children: RefCell::new(vec![Rc::clone(&leaf)])
    // });

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    // 此时无实际弱引用
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        // 创建父节点弱引用
        // leaf 强引用 + 1
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        // 作用域离开后自动清除
    }

    // 获取父节点弱引用
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
