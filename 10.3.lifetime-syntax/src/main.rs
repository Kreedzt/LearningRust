use std::fmt::Display;

// 当定义这个函数时, `并不知道` 传递给函数的具体值, 所以也不知道到底是 if 还是 else 会执行
// 我们也不知道传入的引用的 `具体声明周期`, 所以也不能观察作用域来确定返回的引用是否总是有效
// 借用检查器自身同样也无法确定, 因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的.
// fn longest(x: &str, y:&str) -> &str {
//     // |                                ^ expected lifetime parameter
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 现在函数签名表明对于某些生命周期 ~'a~, 函数会获取 2 个参数, 他们都是与生命周期 ~'a~ 存在的一样长的字符串 slice
// 实际含义是 `longest` 函数返回的引用的生命周期与传入该函数中指定生命周期参数时, 我们并没有改变任何传入值或返回值的生命周期
// 而是指出任何不满足这个约束条件的值都将被借用检查器拒绝.
// *注意*: `longest` 函数并不需要知道 `x` 和 `y` 具体会存在多久, 而只需要知道有某个可以被 `'a` 替代的作用域将会满足这个签名
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 指定生命周期参数的正确方式依赖函数实现的具体功能.
// eg: 如果将 ~longest~ 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice, 就不需要为参数 ~y~ 指定 一个生命周期.
// 即使我们为返回值指定了生命周期参数 `'a`, 这个实现编译失败, 因为返回值的生命周期与参数完全没有关联
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     // 返回值的生命周期与参数完全没有关联. 这里是会出现的信息
//     // result 在 longest函数的结尾将离开作用域并被清理, 我们尝试从函数返回一个 `result` 的引用.
//     // 无法指定生命周期参数来改变悬垂引用, 而且 Rust 也不允许我们创建一个悬垂引用.
//     // 在这种情况, 最好的解决方案是返回一个由所有权的数据类型而不是一个引用, 这样函数调用者复杂清理这个值
//     result.as_str()
// }

// 存放引用的结构体, 定义需要生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 不引用任何值
    fn level(&self) -> i32 {
        3
    }
}

// 适用第三条生命周期省略规则
impl <'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention place: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // 被引用的对象比它的引用者存在的时间更短
    //     //|         ^^^^^^ borrowed value does not live long enough
    // } // 该行结束时离开作用域
    //  - borrowed value needs to live until here
    
    // println!("r: {}", r);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }
    // 尝试在离开作用域后使用 result
    // error[E0597]: `string2` does not live long enough
    // string1 理论上更长, 尚未离开作用域, 但是函数使用了相同的生命周期 'a
    // println!("The longest string is {}", result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentance = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentance
    };
}
