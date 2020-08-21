use trait_objects::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 85,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    // run 并不需要知道各个组件的具体类似是什么. 它并不是检查组件是谁的示例
    // 通过指定 `Box<dyn Draw>` 作为 `components` vector 中值的类型
    // 我们就定义了 `Screen` 为需要可以在其上调用 `draw` 方法的值
    screen.run();

    // 尝试使用一种没有实现 trait 对象的 trait 的类型
    // let screen = Screen {
    //     components: vec![
    //         // error[E0277]: the trait bound `std::string::String: trait_objects::Draw` is not satisfied
    //         Box::new(String::from("Hi"))
    //     ]
    // };
}
