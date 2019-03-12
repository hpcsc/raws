use handlers::common::find_profile_with_name;
use handlers::common::get_value_of_tuple;
use handlers::common::get_assume_settings;
use handlers::common::compose;
use handlers::common::load_ini;
use ini::ini::Properties;
use ini::Ini;
use config::{ SetConfig };
use std::error::Error;

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

fn set_default_assume_settings(file: &Ini, (role_arn, source_profile): (&String, &String)) -> Ini {
    let mut output = file.clone();
    output.set_to(Some("default"), "role_arn".to_owned(), role_arn.to_string());
    output.set_to(Some("default"), "source_profile".to_owned(), source_profile.to_string());
    output
}

fn set_assume_profile(config_file: &Ini, credentials_file: &Ini, selected_profile: &String) -> Result<(Ini, Ini), String> {
    let find_result = find_profile_with_name(config_file, selected_profile)
    .and_then(compose(get_value_of_tuple, get_assume_settings));

    match find_result {
        Some(settings) => {
            let updated_config_file = set_default_assume_settings(config_file, settings);
            Ok((updated_config_file, credentials_file.clone()))
        },
        None => Err("".to_owned())
    }
}

fn set_default_settings(file: &Ini, (aws_access_key_id, aws_secret_access_key): (&String, &String)) -> Ini {
    let mut output = file.clone();
    output.set_to(Some("default"), "aws_access_key_id".to_owned(), aws_access_key_id.to_string());
    output.set_to(Some("default"), "aws_secret_access_key".to_owned(), aws_secret_access_key.to_string());
    output
}

fn get_profile_settings(properties: &Properties) -> Option<(&String, &String)> {
    let aws_access_key_id = properties.get("aws_access_key_id");
    let aws_secret_access_key= properties.get("aws_secret_access_key");
    if let (Some(key_id), Some(access_key)) = (aws_access_key_id, aws_secret_access_key) {
        return Some((key_id, access_key))
    }

    None
}

fn set_profile(config_file: &Ini, credentials_file: &Ini, selected_profile: &String) -> Result<(Ini, Ini), String> {
    let find_result = find_profile_with_name(credentials_file, selected_profile)
        .and_then(compose(get_value_of_tuple, get_profile_settings));

    match find_result {
        Some(settings) => {
            let updated_config_file = set_default_assume_settings(config_file, (&"".to_owned(), &"".to_owned()));
            let updated_credentials_file = set_default_settings(credentials_file, settings);
            Ok((updated_config_file, updated_credentials_file))
        }
        None => Err("profile not found in both config and credentials file".to_owned())
    }
}

pub fn handle(config: SetConfig,
              mut output: impl FnMut(String) -> (),
              mut choose_profile: impl FnMut(Vec<String>) -> Result<String, Box<Error>>,
              mut write_to_file: impl FnMut(Ini, &String) -> Result<(), Box<Error>>)
              -> Result<(), Box<Error>> {
    let config_file = load_ini(&config.config_path)?;
    let credentials_file = load_ini(&config.credentials_path)?;

    let mut profiles = get_all_profile_names_except_default(&credentials_file);
    profiles.extend(get_all_profile_names_except_default(&config_file));

    let selected_profile = choose_profile(profiles)?;

    let set_result = set_assume_profile(&config_file, &credentials_file, &selected_profile)
                        .or_else(
                     |_| set_profile(&config_file, &credentials_file, &selected_profile))
                        .map_err(|e| e.into());

    set_result.and_then(|(updated_config_file, updated_credentials_file)| {
        write_to_file(updated_config_file, &config.config_path)?;
        write_to_file(updated_credentials_file, &config.credentials_path)?;
        Ok(output("profile set successfully".to_owned()))
    })
}
