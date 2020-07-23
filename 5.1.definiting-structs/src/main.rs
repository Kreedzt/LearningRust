struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct User2 {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");
    
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // 指定了剩余未显示设定值的字段应有与给定治理对应相同的值
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user2 = User2 {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1
    }
}

fn build_user(email: String, username: String) -> User {
    // 隐式返回
    User {
        // email: email,
        // username: username,
        // 字段初始化简写语法
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
