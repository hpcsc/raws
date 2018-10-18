#[macro_use]
extern crate clap;
use clap::ArgMatches;
use clap::{ App };

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = parse_config(&matches);

    let result = match config {
        Config::Get(config) => get_current_profile(config),
        Config::Set(config) => set_profile(config),
        _ => Ok(()),
    };

    match result {
        Err(message) => println!("{}", message),
        _ => (),
    }
}

enum Config {
    Get(GetConfig),
    Set(SetConfig),
    Invalid,
}

struct GetConfig {
    credentials_path: String,
    config_path: String,
}

struct SetConfig {
    credentials_path: String,
    config_path: String,
    pattern: String,
}

fn get_arg(matches: &ArgMatches, arg_name: &str, default_value: &str) -> String {
    matches.value_of(arg_name)
           .unwrap_or(default_value)
           .to_string()
}

fn get_credentials_path(matches: &ArgMatches) -> String {
    get_arg(matches, "credentials-path", "~/.aws/credentials")
}

fn get_config_path(matches: &ArgMatches) -> String {
    get_arg(matches, "config-path", "~/.aws/config")
}

fn parse_config(matches: &ArgMatches) -> Config {
    match matches.subcommand() {
        ("get", Some(m)) => Config::Get(GetConfig {
                                credentials_path: get_credentials_path(m),
                                config_path: get_config_path(m)
                            }),
        ("set", Some(m)) => Config::Set(SetConfig {
                                credentials_path: get_credentials_path(m),
                                config_path: get_config_path(m),
                                pattern: get_arg(m, "PROFILE_PATTERN", ""),
                            }),
        _ => Config::Invalid
    }
}

fn get_current_profile(config: GetConfig) -> Result<(), String> {
    println!("get called {} {}", config.credentials_path, config.config_path);
    Ok(())
}

fn set_profile(config: SetConfig) -> Result<(), String> {
    println!("set called {} {} {}", config.credentials_path, config.config_path, config.pattern);
    Ok(())
}
