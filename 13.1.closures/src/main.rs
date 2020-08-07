use std::{thread, time::Duration};

struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // Cacher 实例将 `Some(1)` 保存进 self.value, 在这之后, 无论传递怎么值, 都是 1
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_expensive_caculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 定义闭包
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // 自动推断第一次类型
    // let n = example_closure(5);

    // let x = 4;
    // 闭包访问外部变量 x
    // let equal_to_x = |z| z == x;

    // 函数无法访问
    // can't capture dynamic environment in a fn item; use the || { ...
    // } closure form instead
    // fn equal_to_x(z: i32) -> bool { z == x };
    // let y = 4;

    // assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    // 使用 move 强制获取环境值的所有权
    // x 被移动进了闭包, 由于闭包使用 `move`, 接着闭包获取了 `x` 的所有权.
    // 同时main 中就不在允许在 `println!` 语句中使用 `x` 了.
    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);
    // ^ value borrowed here after move

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    
    let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if (intensity < 25) {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} siteups!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remeber to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}

// fn generate_workout(intensity: u32, random_number: u32) {
//     // let expensive_result =
//     //     simulated_expensive_caculation(intensity);

//     // let expensive_closure = |num| {
//     //     println!("calculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num
//     // }

//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if (intensity < 25) {
//         println!(
//             "Today, do {} pushups!",
//             expensive_closure(intensity)
//         );
//         println!(
//             "Next, do {} siteups!",
//             expensive_closure(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remeber to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }

// fn generate_workout(intensity: u32, ramdom_number: u32) {
//     let expensive_result =
//         simulated_expensive_caculation(intensity);

//     if (intensity < 25) {
//         println!(
//             "Today, do {} pushups!",
//             expensive_result
//         );
//         println!(
//             "Next, do {} siteups!",
//             expensive_result
//         );
//     } else {
//         if ramdom_number == 3 {
//             println!("Take a break today! Remeber to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes",
//                 expensive_result
//             );
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_caculation(intensity)
//         );
//         println!(
//             "Next do {} siteups!",
//             simulated_expensive_caculation(intensity)
//         )
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_caculation(intensity)
//             )
//         }
//     }
// }

#[test]
fn call_width_different_values() {
    let mut c  = Cacher::new(|a| a);

    // Cacher 实例将 `Some(1)` 保存进 self.value, 在这之后, 无论传递怎么值, 都是 1
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
