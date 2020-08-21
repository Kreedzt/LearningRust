// 状态模式: 开始
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             // 1. Post: add a Draft, 当前 State 为 Draft
//             state: Some(Box::new(Draft {})),
//             content: String::new()
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         // 2. Post: add_text
//         println!("Triggered add_text, {}.", text);
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         // ""
//         println!("Get Post content.");
//         // 9. 传递self(当前作用域)
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review(&mut self) {
//         // `take` 方法将 `state` 字段中的 `Some` 值去除并留下一个 `None`
//         // 3. Post: 拿取当前 state(Draft)
//         println!("Triggered Post request_review.");
//         if let Some(s) = self.state.take() {
//             // 4. Post: 拿取 new PendingReview(Return by Draft), content 默认返回 ""(更新state)
//             // 当前 State 为 PendingReview
//             println!("After Triggered Post request_review.");
//             self.state = Some(s.request_review())
//         }
//     }

//     pub fn approve(&mut self) {
//         // 6. 拿取当前 state(PendingReview)
//         println!("Triggered Post approve.");
//         if let Some(s) = self.state.take() {
//             // 7. 拿取当前 PendingReview 的 approve()
//             // 当前 State 为 Published
//             println!("After Triggered Post approve.");
//             self.state = Some(s.approve())
//         }
//     }
// }

// trait State {
//     // 为了消费老状态, 需要获取状态值的所有权
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     // 存在默认返回, 无需为 `Draft` 和 `PendingReview` 结构体实现 `content`
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         // 返回新创建, 代表处于待审核状态
//         // 5. Draft: return new PendingReview
//         println!("Triggered Draft request_review.");
//         Box::new(PendingReview {})
//     }

//     // 6. Draft: return self
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         println!("Triggered Draft approve.");
//         self
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     // 不进行任何转换, 返回自身
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         println!("Triggered PendingReview request_review.");
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         // 8. 返回 new Published
//         println!("Triggered PendingReview approve.");
//         Box::new(Published {})
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         println!("Triggered Published request_review.");
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         println!("Triggered Published approve.");
//         self
//     }

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         println!("Triggered Published content.");
//         // 10. 返回获取传递 Post 的 content
//         &post.content
//     }
// }
// 状态模式: 结束

// 将状态和行为编码为类型
pub struct Post {
    content: String
}

pub struct DraftPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        // 1: 返回 DraftPost
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        // 5: 只有 Post 有 content() 方法, 保证非最后一步无法访问 content
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        // 2: 添加字符串
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        // 3: 返回 PendingReviewPost, 传递 content
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        // 4. 返回 Post, 传递 content
        Post {
            content: self.content
        }
    }
}
