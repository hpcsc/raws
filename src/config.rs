use clap::ArgMatches;

pub enum Config {
    Get(GetConfig),
    Set(SetConfig),
}

pub struct GetConfig {
    pub credentials_path: String,
    pub config_path: String,
}

pub struct SetConfig {
    pub credentials_path: String,
    pub config_path: String,
    pub pattern: String,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Option<Config> {
        match matches.subcommand() {
            ("get", Some(m)) => Some(Config::Get(GetConfig {
                                    credentials_path: get_credentials_path(m),
                                    config_path: get_config_path(m)
                                })),
            ("set", Some(m)) => Some(Config::Set(SetConfig {
                                    credentials_path: get_credentials_path(m),
                                    config_path: get_config_path(m),
                                    pattern: get_arg(m, "PROFILE_PATTERN", ""),
                                })),
            _ => None
        }
    }
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
