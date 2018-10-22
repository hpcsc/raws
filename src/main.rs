#[macro_use]
extern crate clap;
extern crate raws;
use clap::{ App };

use raws::config::{ Config };
use raws::handlers;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
                    .version(VERSION)
                    .get_matches();

    let config = Config::new(&matches).unwrap();

    let result = match config {
        Config::Get(config) => handlers::get_current_profile(config),
        Config::Set(config) => handlers::set_profile(config),
    };

    result.unwrap_or_else(|error_message| {
        println!("== Error: {}", error_message);
    })
}
