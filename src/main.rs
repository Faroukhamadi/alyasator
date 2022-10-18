use std::process::{self};

use alyasator::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|e| {
        eprintln!("Problem getting answers: {e}");
        process::exit(1);
    });

    if let Err(e) = alyasator::run(config) {
        eprintln!("❌ Application error: {e}");
        process::exit(1);
    }
}
