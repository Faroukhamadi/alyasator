use std::process;

use alyasator::{Config, run};

fn main() {
    if let Err(e) = run(Config::build()) {
        eprintln!("❌ Application error: {}", e);
        process::exit(1);
    }
}
