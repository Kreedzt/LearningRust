pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl <'a, T> LimitTracker<'a, T>
where T: Messager {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >- 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // struct MockMessenger {
    //     sent_messages: Vec<String>
    // }

    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger {
    //             sent_messages: vec![]
    //         }
    //     }
    // }

    // impl Messager for MockMessenger {
    //     // error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
    //     // 因为 send 方法获取了 self 的不可变引用
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));
    //     }
    // }

    // #[test]
    // fn it_sends_an_over_75_percent_warning_message() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    //     limit_tracker.set_value(80);

    //     assert_eq!(mock_messenger.sent_messages.len(), 1);
    // }

    struct MockMessenger {
        // 内部可变, send 可以修改 sent_message 并储存
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messager for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }

        // fn send(&self, message: &str) {
        //     let mut one_borrow = self.sent_messages.borrow_mut();
        //     // ---- tests::it_sends_an_over_75_percent_warning_message stdout ----
        //     // thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at 'already borrowed: BorrowMutError', C:\Users\Ken Zhao\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\src\libcore\cell.rs:877:9
        //     let mut two_borrow = self.sent_messages.borrow_mut();
            
        //     one_borrow.push(String::from(message));
        //     two_borrow.push(String::from(message));
        // }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
