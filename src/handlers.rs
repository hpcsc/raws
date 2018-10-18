use super::config::{ GetConfig, SetConfig };

pub fn get_current_profile(config: GetConfig) -> Result<(), String> {
    println!("get called {} {}", config.credentials_path, config.config_path);
    Ok(())
}

pub fn set_profile(config: SetConfig) -> Result<(), String> {
    println!("set called {} {} {}", config.credentials_path, config.config_path, config.pattern);
    Ok(())
}
