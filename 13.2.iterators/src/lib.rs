// 创建自定义迭代器
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// 使用自定义迭代器的其他 Iterator trait 方法
#[test]
fn using_other_iterator_trait_methods() {
    // (2, 3), (4, 5)
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    // Res: (1, 2), (2, 3), (3, 4), (4, 5), (5, None)[此项未产生] // zip()
    // Res: 2, 6, 12, 20 // map()
    // Res: 6, 12 // filter()
    // Res: 18 // sum()

    assert_eq!(18, sum);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

// 使用闭包获取环境
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // v1_iter 是可变的, 代码消费/使用了迭代器. 每一个 `next` 调用都会从迭代器中消费一个项
    let mut v1_iter = v1.iter();
    // 使用 `for` 循环时无需使 `v1_iter` 可变因为 `for` 循环会获取 `v1_iter` 的所有权并在后台使 `v1_iter` 可变

    // 从 `next` 调用中得到的值是 vector 的 不可变引用. `iter` 方法生成一个不可变引用的迭代器.
    // 如果我们需要一个获取 `v1` 所有权并返回拥有所有权的迭代器, 则可以调用 `into_iter` 而不是 `iter`
    // 类似地, 如果希望迭代可变引用, 则可以调用 `iter_mut` 而不是 `iter`
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
