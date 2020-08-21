fn get_op() -> Option<i32> {
    return Some(5);
}

fn main() {
    let some_operation_value = get_op();

    // 尝试在要求不可反驳模式的地方使用可反驳模式
    // error[E0005]: refutable pattern in local binding: `None` not covered
    // let Some(x) = some_operation_value;

    // if let Some(x) = some_operation_value {
    //     println!("{}", x);
    // }

    // 将不可反驳模式用于 if let 是没有意义的
    // warning: irrefutable if-let pattern
    if let x = 5 {
        println!("{}", x);
    }
}
