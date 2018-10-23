use handlers::common::load_ini;
use ini::ini::Properties;
use ini::Ini;
use config::{ GetConfig };

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

fn find_section_with_same_access_key<'a>(credentials_file: &'a Ini) -> impl Fn(&String) -> Option<(&'a Option<String>, &'a Properties)> {
    move |default_access_key_id: &String| {
            credentials_file.iter().find(|(section, properties)| {
            section_is_not_default(section) &&
            section_has_same_access_key_id(default_access_key_id, properties)
        })
    }
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

pub fn handle(config: GetConfig) -> Result<(), String> {
    let credentials_file = load_ini(&config.credentials_path)?;
    let config_file = load_ini(&config.config_path)?;

    let section_with_same_access_key = find_default_section(&credentials_file)
                                    .and_then(get_default_access_key_id)
                                    .and_then(find_section_with_same_access_key(&credentials_file));
    println!("{:?}", section_with_same_access_key);
    Ok(())
}
