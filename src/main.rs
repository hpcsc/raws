#[macro_use]
extern crate clap;
use clap::ArgMatches;
use clap::{ App };

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("get", Some(m)) => handle_get(m),
        ("set", Some(m)) => handle_set(m),
        _ => Ok(()),
    };
}

fn handle_get(matches: &ArgMatches) -> Result<(), String> {
    println!("get called {} {}", matches.value_of("credentials-path").unwrap(), matches.value_of("config-path").unwrap());
    Ok(())
}

fn handle_set(matches: &ArgMatches) -> Result<(), String> {
    println!("set called {} {}", matches.value_of("credentials-path").unwrap(), matches.value_of("config-path").unwrap());
    Ok(())
}
