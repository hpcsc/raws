#[macro_use]
extern crate clap;
extern crate raws;
extern crate ini;
extern crate shellexpand;

use clap::{App, AppSettings};
use ini::Ini;

use raws::config::Config;
use raws::handlers::{get, set, fzf};
use std::error::Error;
use shellexpand::tilde;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn write_to_file(file: Ini, output_path: &String) -> Result<(), Box<Error>> {
    file.write_to_file(tilde(output_path).to_string()).map_err(|e| e.into())
}

fn execute_handler(config: Config) -> Result<String, Box<Error>> {
    match config {
        Config::Get(config) => get::handle(config),
        Config::Set(config) => set::handle(config, fzf::choose_profile, write_to_file),
    }
}

fn print_result(result: Result<String, Box<Error>>) {
    match result {
        Ok(ref message) if !message.is_empty() => println!("{}", message),
        Err(error) => println!("== Error: {}", error),
        _ => ()
    };
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let  app = App::from_yaml(yaml)
        .version(VERSION)
        .setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();
    let config = Config::new(&matches).unwrap();

    let result = execute_handler(config);

    print_result(result);
}
