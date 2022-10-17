use std::process;

use alyasator::{append_to_rc, Config};

fn main() {
    // TODO: reset this later
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem getting answers: {err}");
        process::exit(1);
    });

    append_to_rc(config).unwrap_or_else(|err| {
        eprintln!("Problem appending to rc: {err}");
        process::exit(1);
    })

    // TODO: reset this later
    // println!("config's shell: {}", config.shell);
    // println!("config's is_permanent: {}", config.is_permanent);
    // println!("config's alias: {}", config.alias);
    // println!("config's original: {}", config.original);
}
