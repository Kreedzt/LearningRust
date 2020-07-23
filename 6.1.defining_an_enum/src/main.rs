enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called Message inner fn");
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // error[E0277]: cannot add `Option<i8>` to `i8`
    // let sum = x + y;
}

fn route(ip_type: IpAddrKind) {
    
}
