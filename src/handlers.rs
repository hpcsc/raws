use ini::ini::Properties;
use shellexpand::tilde;
use ini::ini::Error::{ Io, Parse };
use ini::Ini;
use super::config::{ GetConfig, SetConfig };

fn section_is_not_default(section: &Option<String>) -> bool {
    match section {
        Some(name) => name != "default",
        None => false
    }
}

fn section_has_same_access_key_id(default_access_key_id: &String, properties: &Properties) -> bool {
    match properties.get("aws_access_key_id") {
        Some(value) => value == default_access_key_id,
        None => false
    }
}

fn find_section_with_same_access_key<'a>(default_access_key_id: &String, credentials_file: &'a Ini) -> Option<(&'a Option<String>, &'a Properties)> {
    credentials_file.iter().find(|(section, properties)| {
        section_is_not_default(section) &&
        section_has_same_access_key_id(default_access_key_id, properties)
    })
}

fn get_default_access_key_id<'a>((_, props): (&Option<String>, &'a Properties)) -> Option<&'a String> {
    props.get("aws_access_key_id")
}

fn find_default_section<'a>(credentials_file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    credentials_file.iter().find(|(section, _)|
        match section {
            Some(n) => n == "default",
            None => false,
        })
}

pub fn get_current_profile(config: GetConfig) -> Result<(), String> {
    let credentials_file = load_ini(&config.credentials_path)?;
    let config_file = load_ini(&config.config_path)?;

    let default_access_key_id = find_default_section(&credentials_file)
                                    .and_then(get_default_access_key_id);
    let section_with_same_access_key = find_section_with_same_access_key(default_access_key_id.unwrap(), &credentials_file);
    println!("{:?}", section_with_same_access_key);
    println!("get called {} {} {}", config.credentials_path,
                                    config.config_path,
                                    credentials_file.section(Some("default"))
                                                    .unwrap()
                                                    .get("aws_access_key_id")
                                                    .unwrap());
    Ok(())
}

pub fn set_profile(config: SetConfig) -> Result<(), String> {
    println!("set called {} {} {}", config.credentials_path, config.config_path, config.pattern);
    Ok(())
}

fn load_ini(path: &String) -> Result<Ini, String> {
    let expanded_path = tilde(path).to_string();

    match Ini::load_from_file(expanded_path) {
        Ok(f) => Ok(f),
        Err(Io(_)) => Err(format!("failed to load file {}", path)),
        Err(Parse(_)) => Err(format!("invalid file {}", path)),
    }
}
