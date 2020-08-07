//! # Art
//!
//! 一个描述美术信息的库

// 使用 pub use 重导出
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

// 没有在首页列出
pub mod kinds {
    /// 采用 RGB 色彩模式的主要颜色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    /// 采用 RGB 色彩模式的次要颜色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 等量的混合两个主要颜色
    /// 来创建一个次要颜色
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // ...
        SecondaryColor::Orange
    }
}

// //! # My Crate
// //!
// //! `my_crate` 是一个使得特定计算更方便的
// //! 工具集合

// /// 将给定的数字加一
// /// # Examples
// /// ```
// /// let arg = 5;
// /// let answer = publishing_to_crates_io::add_one(arg);
// ///
// /// assert_eq!(6, answer);
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }
