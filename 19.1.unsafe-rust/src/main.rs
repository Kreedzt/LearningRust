use std::slice;
unsafe fn dangerous() {}

// 使用 mut 标明可变性
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 任何读写全局变量必须在 unsafe 块中
    unsafe {
        COUNTER += inc;
    }
}

// FFI
// ABI 集成 C 标准库的 `abs` 函数
extern "C" {
    fn abs(input: i32) -> i32;
}

// 全局变量
// 类型: &'static str
static HELLO_WORLD: &str = "Hello, world!";

// 实现不安全 trait
unsafe trait Foo {
    // 方法
}

unsafe impl Foo for i32 {
    // 方法实现
}

fn main() {
    let mut num = 5;

    // 此处无需使用 unsafe, 只是不能再不安全块之外 `解引用` 裸指针
    // 创建不可变 裸指针
    let r1 = &num as *const i32;
    // 创建可变 裸指针
    let r2 = &mut num as *mut i32;
    // *注意*: 此处创建了 可变 与 不可变指针, 可能会产生数据竞争问题

    let address = 0x012345usize;
    // 不能确定其有效性的裸指针.
    let r = address as *const i32;

    // 解引用只能在 unsafe 中进行
    unsafe {
        // 创建一个指针不会造成任何危险, 只有访问其指向的值才可能遇到无效的值
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }


    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 通过任意内存地址创建 slice
    // let slice: &[i32] = unsafe {
    //     slice::from_raw_parts_mut(r, 10000)
    // };

    unsafe {
        println!("Absolute value of -32 according to C: {}", abs(-3));
    }

    // 全局变量访问
    println!("name is: {}", HELLO_WORLD);

    // 调用修改全局变量
    add_to_count(3);

    // 任何读写全局变量必须在 unsafe 块中
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = slice.len();
    // 访问裸指针
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // error[E0499]: cannot borrow `*slice` as mutable more than once at a time
    // (&mut slice[..mid],
    // &mut slice[mid..])

    unsafe {
        // 从裸指针和长度创建一个 slice
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}
