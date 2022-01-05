use std::{env, fs, process};

// https://kaisery.github.io/trpl-zh-cn/ch12-03-improving-error-handling-and-modularity.html
fn main() {
    let args: Vec<String> = env::args().collect();
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

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    println!("{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

