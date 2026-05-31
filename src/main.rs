use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    // run(config);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
struct Config {
    query: String,
    file_path: String,
    pub ignore_case: bool,
}

// fn parse_config(args: &[String]) -> Config {
//     // clone() can refactor to Closures
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

impl Config {
    //build is return Result
    //new -> build refactor
    fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}