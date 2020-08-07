use std::{env, process};
use improving_out_io_project::Config;


fn main() {
    // 改为迭代器
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = improving_out_io_project::run(config) {
        // 错误输出不会重定向到文件
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
