#[macro_use]
extern crate clap;
extern crate raws;
extern crate ini;

use clap::App;
use ini::Ini;

use raws::config::Config;
use raws::handlers::{get, set, fzf};
use std::error::Error;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn write_to_file(file: Ini, output_path: &String) -> Result<(), Box<Error>> {
    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
        .version(VERSION)
        .get_matches();

    let config = Config::new(&matches).unwrap();

    let result = match config {
        Config::Get(config) => get::handle(config),
        Config::Set(config) => set::handle(config, fzf::choose_profile, write_to_file),
    };

    match result {
        Ok(ref message) if !message.is_empty() => println!("{}", message),
        Err(error) => println!("== Error: {}", error),
        _ => ()
    };
}
