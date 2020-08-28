use std::io::prelude::*;
use std::fs;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use final_project_a_web_server::ThreadPool;

fn main() {
    // 监听 TCP 链接
    // 此场景中 `bind` 类似于 `new` 函数, 返回一个新的 TcpListener 实例
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 此处遍历的是 *链接尝试* 而不是链接, 所以可能失败
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     // println!("Connection established!");
    //     // 读取请求
    //     handle_connection(stream);
    // }

    // 为每一个流新建一个线程
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     // 创建新线程并在其中运行闭包的代码.
    //     // 不会阻塞但是会无限制创建线程导致系统崩溃
    //     thread::spawn(|| {
    //         handle_connection(stream);
    //     });
    // }

    // 为有限数量的线程创建一个类似的接口
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// steam 参数是可变的, 因为 TcpStream 实例在内部记录了所返回的数据
// 可能读取了多于我们请求的数据并保存它们以备下一次请求数据
// 因此需要 `mut` 因为其内部状态可能改变
fn handle_connection(mut stream: TcpStream) {
    // 创建 512 字节的缓冲区
    let mut buffer = [0; 512];

    // 调用读取方法读取字节并放入缓冲区中
    stream.read(&mut buffer).unwrap();

    // 将缓冲区的字节转换为字符串并打印出来.
    // 该函数获取一个 `&[u8]` 并产生一个 `String`.
    // 函数名的 "lossy" 部分来源于当其遇到无效的 UTF-8 序列时的行为:
    // 使用 � `U+FFFD REPLACEMENT CHARACTER` 来代替无效序列.
    // 可能会载缓冲区的剩余部分看到这些字符, 因为他们没有被请求数据填满
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 编写成功响应
    // 存放成功响应的数据
    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    // 返回真正的 HTML
    // let contents = fs::read_to_string("hello.html").unwrap();

    // let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // println!("Response: \n{}", response);

    // 转为 streeam 需要的 &[u8] 字节类型, 并发送给链接
    // write 操作可能会失败
    // stream.write(response.as_bytes()).unwrap();
    // flush 会等待并阻塞程序执行直到所有字节都被写入链接中;
    // stream.flush().unwrap();
    // 此时可以成功响应

    // 增加 if 判断
    // b 转为字节字符串
    let get = b"GET / HTTP/1.1\r\n";

    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();

    //     let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    //     // 其他请求
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();

    //     let response = format!("{}{}", status_line, contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    // 模拟慢请求
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // 少量代码重构
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
