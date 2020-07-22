use std::io;

fn main() {
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // Error: 抛出编译期错误
    // if number {
    //     println!("number was true");
    // }

    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not dibisible by 4, 3, or 2");
    // }

    // let condition = true;

    // let number = if condition {
    //     5
    // } else {
    //     6
    // };
    
    // println!("The value of number is: {}", number);

    // let condition = true;

    // 编译期错误, 类型不匹配
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };

    // println!("The value of number is: {}", number);

    // loop {
    //     println!("again");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("LISTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    // // for 遍历数组
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // // rev 翻转
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }

    // println!("LIFTOFF!!!");

    
    // start_converse_fn();
    generate_fibonacci(10);
    println!("Exit.");
}

fn generate_fibonacci(n: u32) {
    let mut cur = 1;
    // 前2项
    let mut prev_b: u32 = 1;
    // 前1项
    let mut prev_c: u32 = 1;
    let mut cur_val: u32 = 1;
    while cur < n {
        if cur > 2 {
            cur_val = prev_b + prev_c;
            prev_b = prev_c;
            prev_c = cur_val;
        }
        
        println!("Current val: {}", cur_val);
        cur += 1;
    }
}

fn start_converse_fn() {
    let mut input_str = String::new();
    let input_c: f64;
    let input_f: f64;

    loop {
        println!("Input °C");
        
        io::stdin().read_line(&mut input_str).expect("Error format");

        input_c = match input_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your Input: {}°C, {}°F", input_c, converse_temperature_f(input_c));
        break;
    }

    loop {
        println!("Input °F");
        input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("Error format");

        input_f = match input_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error format");
                0.0
            },
        };

        println!("Your Input: {}°F, {}°C", input_f, converse_temperature_c(input_f));
        break;
    }
}

fn converse_temperature_f(t: f64) -> f64 {
    t * 1.8 + 32.0
}

fn converse_temperature_c(t: f64) -> f64 {
    (t - 32.0) / 1.8
}
