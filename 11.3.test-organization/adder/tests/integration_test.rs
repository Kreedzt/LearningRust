use adder;

// 将 common作文模块以便在任何集成测试文件中使用
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
