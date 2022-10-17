use std::process;

use alyasator::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem getting answers: {err}");
        process::exit(1);
    });

    println!("config's shell: {}", config.shell);
    println!("config's is_permanent: {}", config.is_permanent);
    println!("config's alias: {}", config.alias);
    println!("config's original: {}", config.original);
}
