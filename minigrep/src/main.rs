use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(| err| {
        eprintln!("get config error: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("run error: {err}");
        process::exit(1);
    }
}
