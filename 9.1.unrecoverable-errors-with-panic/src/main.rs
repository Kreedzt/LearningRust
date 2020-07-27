fn main() {
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // libcore/slice/mod.rs
    // index out of bounds: the len is 3 but the index is 99
    v[99];

    // 设置 RUST_BACKTRACE时, 发现第 12 行对应 main.rs, 使用 --release减少输出
}
