#[macro_use]
extern crate clap;
extern crate raws;
use clap::{ App };

use raws::config::{ Config };
use raws::handlers::{ get, set, fzf };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn display_result(message: String) -> () {
    println!("{}", message)
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
                    .version(VERSION)
                    .get_matches();

    let config = Config::new(&matches).unwrap();

    let result = match config {
        Config::Get(config) => get::handle(config, display_result),
        Config::Set(config) => set::handle(config, display_result, fzf::choose_profile),
    };

    result.unwrap_or_else(|error_message| {
        println!("== Error: {}", error_message);
    })
}
