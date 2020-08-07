fn main() {
    let v1 = vec![1, 2, 3];

    // 创建迭代器, 但未操作
    let v1_iter = v1.iter();

    // 一旦 `for` 循环开始使用 `v1_iter`, 才开始迭代
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];

    // v1.iter().map(|x| x + 1);

    // 调用 collect() 消费迭代器并将结果收集到一个数据结构中
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // assert_eq!(v2, vec![2, 3, 4]);
}
