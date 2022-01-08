use minigrep;
use std::{env, process};

// https://kaisery.github.io/trpl-zh-cn/ch12-03-improving-error-handling-and-modularity.html

// 输出信息保存到文件
// cargo run to poem.txt > output.txt

fn main() {
    let args = env::args();
    // 用"_"代替，不会检查该变量是否使用
    /* let config = match Config::new(&args) {
        Ok(c) => c,
        Err(_) => process::exit(1)
    }; */

    // 使用match
    /* let config = match Config::new(&args) {
         Ok(c) => c,
         Err(e) => {
             println!("Problem parsing arguments: {}", e);
             process::exit(1)
         }
     }; */

    let config = minigrep::Config::new(args).unwrap_or_else(|e| {
        // eprintln 保存输出信息时不会保存eprintln的输出
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // 用if let
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


