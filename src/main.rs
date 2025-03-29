use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static String> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(
            Config { query, file_path }
        )
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query: String = args[1].clone();
//     let file_path: String = args[2].clone();

//     Config { query, file_path }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let config: Config = parse_config(&args);
    
    let config: Config = Config::build(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
