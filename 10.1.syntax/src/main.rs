fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         // error[E0369]: binary operation `>` cannot be applied to type `T`
//         // 25 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//         // std::cmp::PartialOrd 这是一个 trait
//         // 这个错误表明 largest 的函数体不能适用于 T 的所有可能的类型
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T
// }

// struct Point<T, U> {
//     x: T,
//     y: U
// }

// 枚举定义中的泛型
// 标准库
// enum Option<T> {
//     Some(T),
//     None,
// }

// 标准库
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// 方法定义中的泛型
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// 单独实现
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    // 创建新示例
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    
    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // 必须同类型
    // let wont_work = Point { x:  5, y: 4.0 };

    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
