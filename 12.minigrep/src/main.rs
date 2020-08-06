// use std::{fs, env, process, error::Error};
use std::{env, process};
use minigrep::Config;


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

    // let config = parse_config(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);
        // 错误输出不会重定向到文件
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    // 警告: 没有错误处理
    // run(config);
    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {}", e);
        // 错误输出不会重定向到文件
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // let contents = fs::read_to_string(config.filename)
//     //     .expect("Something went wrong reading the file");

//     let contents = fs::read_to_string(config.filename)?;

//     println!("With text:\n{}", contents);

//     Ok(())
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

// fn parse_config(args: &[String]) -> Config {
//     // clone 拥有数据的完整拷贝, 无需管理引用的生命周期
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }
