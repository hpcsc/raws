use handlers::common::load_ini;
use ini::ini::Properties;
use ini::Ini;
use config::{ SetConfig };

pub fn handle(config: SetConfig,
              mut output: impl FnMut(String) -> (),
              mut choose_profile: impl FnMut(Vec<String>) -> String) -> Result<(), String> {
    let config_file = load_ini(&config.config_path)?;
    let credentials_file = load_ini(&config.credentials_path)?;
    let profiles = vec![
        "first_profile".to_owned(),
        "second_profile".to_owned(),
        "profile first_assumed_profile".to_owned(),
        "profile second_assumed_profile".to_owned(),
    ];
    choose_profile(profiles);
    println!("set called {} {} {}", config.credentials_path, config.config_path, config.pattern);
    Ok(())
}
