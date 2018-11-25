use handlers::common::compose;
use handlers::common::load_ini;
use std::io::Error;
use ini::ini::Properties;
use ini::Ini;
use config::{ SetConfig };

fn get_all_profile_names_except_default(file: &Ini) -> Vec<String> {
    let mut profiles: Vec<String> = file.iter()
        .filter_map(|(section, _)|
            match section {
                Some(section_name) if section_name != "default" => Some(section_name.clone()),
                _ => None
            })
        .collect();
    profiles.sort();
    profiles
}

fn find_profile_with_name<'a>(file: &'a Ini, selected_profile: &String) -> Option<(&'a Option<String>, &'a Properties)> {
    file.iter().find(|(section, _)|
        match section {
            Some(section_name) => section_name == selected_profile,
            None => false
        }
    )
}

fn get_value_of_tuple<'a>((_, properties): (&Option<String>, &'a Properties)) -> &'a Properties {
    properties
}

fn get_assume_settings<'a>(properties: &'a Properties) -> Option<(&'a String, &'a String)> {
    let role_arn = properties.get("role_arn");
    let source_profile = properties.get("source_profile");
    if let (Some(arn), Some(profile)) = (role_arn, source_profile) {
        return Some((arn, profile))
    }

    None
}

fn set_default_assume_settings(file: &Ini, (role_arn, source_profile): (&String, &String)) -> Result<Ini, String> {
    let mut output = file.clone();
    output.set_to(Some("default"), "role_arn".to_owned(), role_arn.to_string());
    output.set_to(Some("default"), "source_profile".to_owned(), source_profile.to_string());
    Ok(output)
}

fn set_assume_profile(config_file: &Ini, selected_profile: &String) -> Result<Ini, String> {
    let find_result = find_profile_with_name(config_file, selected_profile)
    .and_then(compose(get_value_of_tuple, get_assume_settings));

    match find_result {
        Some(settings) => set_default_assume_settings(config_file, settings),
        None => Err("".to_owned())
    }
}

pub fn handle(config: SetConfig,
              mut output: impl FnMut(String) -> (),
              mut choose_profile: impl FnMut(Vec<String>) -> String,
              mut write_to_file: impl FnMut(Ini, &String) -> Result<(), String>) -> Result<(), String> {
    let config_file = load_ini(&config.config_path)?;
    let credentials_file = load_ini(&config.credentials_path)?;

    let mut profiles = get_all_profile_names_except_default(&credentials_file);
    profiles.extend(get_all_profile_names_except_default(&config_file));

    let selected_profile = choose_profile(profiles);

    let set_result = set_assume_profile(&config_file, &selected_profile);

    set_result.and_then(|output_file| {
        write_to_file(output_file, &config.config_path)?;
        Ok(output("profile set successfully".to_owned()))
    })
}
