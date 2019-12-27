use std::fs;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub key: String,
    pub filename: String,
    pub case_sensitive: bool,
}

///
/// # Cacher
/// 缓存函数值
pub struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

///
/// # Cacher 实现
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    /// Cacher::new
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.calculation)(arg);
                self.value = Some(value);
                value
            },
        }

    }

}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        args.next();
        let key = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { key, filename , case_sensitive })
    }
}

pub fn search<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(key)).collect()
}

pub fn search_case_insensitive<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&key.to_lowercase()) {
            results.push(line)
        }
    }
    results
}

///
/// # run search
/// ```
/// println!("hell test");
/// ```
pub fn run(config: Config) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.key, &contents)
    } else {
        search_case_insensitive(&config.key, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
