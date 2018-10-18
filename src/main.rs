#[macro_use]
extern crate clap;
extern crate raws;
use clap::{ App };

use raws::config::{ Config };
use raws::handlers;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = Config::new(&matches);

    let result = match config {
        Config::Get(config) => handlers::get_current_profile(config),
        Config::Set(config) => handlers::set_profile(config),
        _ => Ok(()),
    };

    match result {
        Err(message) => println!("{}", message),
        _ => (),
    }
}
