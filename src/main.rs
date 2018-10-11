#[macro_use]
extern crate clap;
use clap::ArgMatches;
use clap::{ App };

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let result = match matches.subcommand() {
        ("get", Some(m)) => handle_get(m),
        ("set", Some(m)) => handle_set(m),
        _ => Ok(()),
    };
}

fn get_arg(matches: &ArgMatches, arg_name: &str, default_value: &str) -> String {
    matches.value_of(arg_name)
           .unwrap_or(default_value)
           .to_string()
}

struct GetConfig {
    credentials_path: String,
    config_path: String,
}

fn handle_get(matches: &ArgMatches) -> Result<(), String> {
    let config = GetConfig {
        credentials_path: get_arg(matches, "credentials-path", "~/.aws/credentials"),
        config_path: get_arg(matches, "config-path", "~/.aws/config"),
    };

    println!("get called {} {}", config.credentials_path, config.config_path);
    Ok(())
}

struct SetConfig {
    credentials_path: String,
    config_path: String,
    pattern: String,
}

fn handle_set(matches: &ArgMatches) -> Result<(), String> {
    let config = SetConfig {
        credentials_path: get_arg(matches, "credentials-path", "~/.aws/credentials"),
        config_path: get_arg(matches, "config-path", "~/.aws/config"),
        pattern: get_arg(matches, "PROFILE_PATTERN", ""),
    };

    println!("set called {} {} {}", config.credentials_path, config.config_path, config.pattern);
    Ok(())
}
