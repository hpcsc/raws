use ini::ini::Properties;
use ini::Ini;
use config::{ SetConfig };

pub fn handle(config: SetConfig) -> Result<(), String> {
    println!("set called {} {} {}", config.credentials_path, config.config_path, config.pattern);
    Ok(())
}
