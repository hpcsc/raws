use handlers::common::{ load_ini, compose };
use ini::ini::Properties;
use ini::Ini;
use config::{ GetConfig };

fn get_access_key_id<'a>(properties: &'a Properties) -> Option<&'a String> {
    properties.get("aws_access_key_id")
}

fn section_has_same_access_key_id(default_access_key_id: &String, properties: &Properties) -> bool {
    match get_access_key_id(properties) {
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

fn get_assume_settings<'a>(properties: &'a Properties) -> Option<(&'a String, &'a String)> {
    let role_arn = properties.get("role_arn");
    let source_profile = properties.get("source_profile");
    if let Some(arn) = role_arn {
        if let Some(profile) = source_profile {
            return Some((arn, profile))
        }
    }

    None
}

fn section_has_same_assume_settings(default_assume_settings: (&String, &String), properties: &Properties) -> bool {
    match get_assume_settings(properties) {
        Some(settings) => default_assume_settings == settings,
        None => false
    }
}

fn find_section_with_same_assume_settings<'a>(config_file: &'a Ini) -> impl Fn((&String, &String)) -> Option<(&'a Option<String>, &'a Properties)> {
    move |default_assume_settings: (&String, &String)| {
            config_file.iter().find(|(section, properties)| {
            section_is_not_default(section) &&
            section_has_same_assume_settings(default_assume_settings, &properties)
        })
    }
}

fn get_value_of_tuple<'a>((_, properties): (&Option<String>, &'a Properties)) -> &'a Properties {
    properties
}

fn section_is_not_default(section: &Option<String>) -> bool {
    match section {
        Some(name) => name != "default",
        None => false
    }
}

fn find_default_section<'a>(file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    file.iter().find(|(section, _)|
        match section {
            Some(n) => n == "default",
            None => false,
        })
}

pub fn handle(config: GetConfig) -> Result<(), String> {
    let credentials_file = load_ini(&config.credentials_path)?;
    let config_file = load_ini(&config.config_path)?;

    let section_with_same_assume_settings = find_default_section(&config_file)
        .and_then(compose(get_value_of_tuple, get_assume_settings))
        .and_then(find_section_with_same_assume_settings(&config_file));
    println!("{:?}", section_with_same_assume_settings);

    let section_with_same_access_key = find_default_section(&credentials_file)
                                    .and_then(compose(get_value_of_tuple, get_access_key_id))
                                    .and_then(find_section_with_same_access_key(&credentials_file));
    println!("{:?}", section_with_same_access_key);
    Ok(())
}