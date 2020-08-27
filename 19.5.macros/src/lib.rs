#[macro_export]
macro_rules! vec {
    // 单边模式: `( $( $x:expr ),* )`
    // $() 捕获符合括号内值以用于替换后的代码
    // $x:expr 匹配 Rust 的任意表达式, 并将表达式记作 $x
    // 对于每个在 => 前面匹配模式中的 $() 的部分,
    // 生成 0 个或多个 在 => 后面 位于 $()* 内的 temp.push()
    ( $( $x: expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
                temp_vec
        }
    };

    // 结果:
    // let mut temp_vec = Vec::new();
    // temp_vec.push(1);
    // temp_vec.push(2);
    // temp_vec.push(3);
    // temp_vec
}


use proc_macro;

// 过程宏例子
// 使用特定宏的占位符
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

