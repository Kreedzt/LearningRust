use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! my name is Pancakes!");
//     }
// }

// 自定义过程宏 - 自定义派生宏
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
