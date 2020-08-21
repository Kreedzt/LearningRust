use oo_design_patterns::{Post};

fn main() {
    // 状态模式测试: 开始
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());
    // 状态模式测试: 结束

    // 将状态和行为编码为类型
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
