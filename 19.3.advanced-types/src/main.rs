use std::io::Error;
use std::fmt;


// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

// 类型别名: 易于编写并在整个 `std::io` 中提供了一致额接口.
// type Result<T> = std::result::Result<T, std::io::Error>;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
// }

type Thunk = Box<dyn Fn() + Send + 'static>;

// never type
// fn bar() -> ! {
    
// }

fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    // 动态大小类型, str 大小无法固定
    // error[E0308]: mismatched types
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

// 被当做下个函数处理
// fn generic<T>(t: T) {
    
// }

// 被当做此处理
// fn generic<T: Sized>(t: T) {
    
// }


// 放宽限制
// 将 ~t~ 参数的类型从 ~T~ 变为了 ~&T~.
// 因为类型可能不能 ~Sized~ 的, 
// 所以需要将其置于某种指针之后, 此处选择了引用
fn generic<T: ?Sized>(t: &T) {
    
}

// fn takes_long_type(f: Thunk) {
    
// }

// fn returns_long_type() -> Thunk {
    
// }

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {

// }

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    
// }
