use std::{fs, env};

fn main() {
    // collect 是一个经常需要注明类型的函数, 因为 Rust 不能推断出想要的类型集合
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Seaching for: {}", query);
    // println!("In file: {}", filename);

    // let (query, filename) = parse_config(&args);
    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // clone 拥有数据的完整拷贝, 无需管理引用的生命周期
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }
