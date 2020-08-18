use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mspc::channel 多个生产者, 单个消费者(multiple producer, single consumer) 的缩写.
    // tx: 发送者 transmitter, rx: 接收者 receiver
    let (tx, rx) = mpsc::channel();

    // 使用 move 转移所有权
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     // 所有权已在发送时转移
    //     // error[E0382]: borrow of moved value: `val`
    //     // println!("val is {}", val);
    // });

    // let recevied = rx.recv().unwrap();
    // println!("Got: {}", recevied);

    // 克隆发送者
    let tx1 = mpsc::Sender::clone(&tx);
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("your"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 当迭代器使用
    for received in rx {
        println!("Got: {}", received);
    }
}
