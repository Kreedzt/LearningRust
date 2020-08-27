// Rust 1.31.0 时, `extern crate` 仍是必须的
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

// 自定义过程宏 - 自定义派生宏
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的的语法树
    // 以便可以进行操作
    // syn crate 将字符串中的 Rust 代码解析称为一个可以操作的数据结构
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // quote 将 syn 解析的数据结构转换回 Rust 代码
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 接收一个表达式, 然后在编译时将表达式转换为一个字符串常量
                // eg: 1 + 2 ===> "1 + 2"
                println!("Hello, Macro! My name is {}", stringify!(#name))
            }
        }
    };

    gen.into()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
