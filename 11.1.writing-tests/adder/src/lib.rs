#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width > other.width && self.height > other.height
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str)-> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        // 移除上限检测, 使得不抛 Panic
        // if value < 1 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        // if value < 1 {
        //     panic!("Guess value must be greater than or qual to 1, got {}.", value);
        // } else if value > 100 {
        //     panic!("Guess value must be less than or qual to 100, got {}.", value);
        // }

        // 对换代码触发bug
        if value < 1 {
            panic!("Guess value must be less than or qual to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or qual to 1, got {}.", value);

        }
        
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn exploration() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"));
    // }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        )
    }

    // #[test]
    // #[should_panic]
    // fn greater_than_100() {
    //     // 并未显示有效错误信息
    //     Guess::new(200);
    // }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        // 并未显示有效错误信息
        Guess::new(200);
    }

    // Result也可以用于测试, 但是不能使用 should_panic宏
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

