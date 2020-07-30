use std::fmt::{Display, Debug};

// ~Summary~ 必须是公有 trait 使得其他 crate 可以实现它, 这也是将 ~pub~ 置于 ~trait~ 之前的原因
pub trait Summary {
    fn summarize_author(&self) -> String;
    
    // fn summarize(&self) -> String;
    // 默认实现
    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        format!("(Read more from{}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait 作为参数, Trait Bound
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 2 个实现了 Summary 的参数
// pub fn notify(item1: impl Summary, item2: impl Summary) {
    
// }

// 2个相同类型
// pub fn notify<T: Summary>(item1: T, item2: T) {
    
// }

// 通过 + 指定多个 trait bound
// pub fn notify(item: impl Summary + Display) {
    
// }

// pub fn notify<T: Summary + Display>(item: T) {
    
// }

// 通过 where 简化 trait bound
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    
// }

// 通过 where 简化 trait bound
// fn some_function<T, U>(t: T, u: U) -> i32
// where T: Display + Clone,
//       U: Clone + Debug {
    
// }

// 返回实现了 trait 的类型
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_books"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false
//     }
// }

// 多类型, 无法过编译
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_books"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false
//         }
//     }
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

// 使用 trait bound 有条件地实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// 可以根据任何实现了特定 trait 的类型有条件地实现 trait
// 标准库
// impl<T: Display> ToString for T {
    
// }


fn main () {
    let tweet = Tweet {
        username: String::from("hourse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };

    println!("New article available! {}", article.summarize());

    // 已实现 Display 的 trait, 调用 to_string()
    let s = 3.to_string();
}

