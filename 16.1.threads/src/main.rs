use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // 此处 join() 会导致没有交替进行
    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 等待创建的线程结束
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     // Rust 无法知道新线程会执行多久, 无法知道 v 的引用是否一直有效
    //     println!("Here's a vector: {:?}", v);
    // });

    // v 不再有效
    // drop(v);

    // move: 强制闭包获取其使用的值得所有权
    let handle = thread::spawn(move || {
        // Rust 无法知道新线程会执行多久, 无法知道 v 的引用是否一直有效
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
