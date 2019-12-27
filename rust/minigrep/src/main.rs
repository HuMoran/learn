use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application run error: {}", e);
        process::exit(1);
    }
}
