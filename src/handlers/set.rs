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
    output.set_to(Some("default"), "role_arn".to_string(), role_arn.to_string());
    output.set_to(Some("default"), "source_profile".to_string(), source_profile.to_string());
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
        None => Err("".to_string())
    }
}

fn set_default_settings(file: &Ini, (aws_access_key_id, aws_secret_access_key): (&String, &String)) -> Ini {
    let mut output = file.clone();
    output.set_to(Some("default"), "aws_access_key_id".to_string(), aws_access_key_id.to_string());
    output.set_to(Some("default"), "aws_secret_access_key".to_string(), aws_secret_access_key.to_string());
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
            let updated_config_file = set_default_assume_settings(config_file, (&"".to_string(), &"".to_string()));
            let updated_credentials_file = set_default_settings(credentials_file, settings);
            Ok((updated_config_file, updated_credentials_file))
        }
        None => Err("profile not found in both config and credentials file".to_string())
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
    if selected_profile.is_empty() {
       return Ok(()) ;
    }

    let set_result = set_assume_profile(&config_file, &credentials_file, &selected_profile)
                        .or_else(
                     |_| set_profile(&config_file, &credentials_file, &selected_profile))
                        .map_err(|e| e.into());

    set_result.and_then(|(updated_config_file, updated_credentials_file)| {
        write_to_file(updated_config_file, &config.config_path)?;
        write_to_file(updated_credentials_file, &config.credentials_path)?;
        Ok(output("profile set successfully".to_string()))
    })
}

#[cfg(test)]
mod tests {
    mod get_all_profile_names_except_default {
        use ini::Ini;
        use handlers::set;

        #[test]
        fn return_all_profile_names_except_default_in_sorted_order() {
            let mut conf = Ini::new();
            conf.with_section(Some("b".to_string())).set("role_arn", "arn_b");
            conf.with_section(Some("default".to_string())).set("role_arn", "arn_default");
            conf.with_section(Some("a".to_string())).set("role_arn", "arn_a");
            conf.with_section(Some("c".to_string())).set("role_arn", "arn_c");

            let profiles = set::get_all_profile_names_except_default(&conf);

            assert_eq!(vec!("a", "b", "c"), profiles)
        }
    }

    mod set_default_assume_settings {
        use ini::Ini;
        use handlers::set;

        #[test]
        fn set_default_profile_with_provided_values() {
            let mut conf = Ini::new();
            conf.with_section(Some("default".to_string())).set("role_arn", "arn_default");
            conf.with_section(Some("default".to_string())).set("source_profile", "source_profile_default");

            let updated_conf = set::set_default_assume_settings(&conf,
                                                                (&"updated_arn".to_string(), &"updated_source_profile".to_string()));

            assert_eq!(Some("updated_arn"), updated_conf.get_from(Some("default"), "role_arn"));
            assert_eq!(Some("updated_source_profile"), updated_conf.get_from(Some("default"), "source_profile"));
        }
    }

    mod set_default_settings {
        use handlers::set;
        use ini::Ini;

        #[test]
        fn set_default_profile_with_provided_values() {
            let mut conf = Ini::new();
            conf.with_section(Some("default".to_string())).set("aws_access_key_id", "default_key_id");
            conf.with_section(Some("default".to_string())).set("aws_secret_access_key", "default_secret_access_key");

            let updated_conf = set::set_default_settings(&conf,
                                                                (&"updated_key_id".to_string(), &"updated_secret_access_key".to_string()));

            assert_eq!(Some("updated_key_id"), updated_conf.get_from(Some("default"), "aws_access_key_id"));
            assert_eq!(Some("updated_secret_access_key"), updated_conf.get_from(Some("default"), "aws_secret_access_key"));
        }
    }

    mod get_profile_settings {
        use std::collections::HashMap;
        use handlers::set::get_profile_settings;

        #[test]
        fn return_some_if_both_access_key_id_and_secret_access_key_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("aws_access_key_id".to_string(), "access_key_id_1".to_string());
            section_properties.insert("aws_secret_access_key".to_string(), "secret_access_key_1".to_string());
            let result = get_profile_settings(&section_properties);

            assert!(result.is_some());
            let (aws_access_key_id, aws_secret_access_key) = result.unwrap();
            assert_eq!("access_key_id_1", aws_access_key_id);
            assert_eq!("secret_access_key_1", aws_secret_access_key);
        }

        #[test]
        fn return_none_if_access_key_id_not_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("aws_secret_access_key".to_string(), "secret_access_key_1".to_string());

            let result = get_profile_settings(&section_properties);

            assert!(result.is_none());
        }

        #[test]
        fn return_none_if_secret_access_key_not_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("aws_access_key_id".to_string(), "access_key_id_1".to_string());

            let result = get_profile_settings(&section_properties);

            assert!(result.is_none());
        }
    }
}
