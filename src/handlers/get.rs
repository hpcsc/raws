use handlers::common::find_profile_with_name;
use handlers::common::get_value_of_tuple;
use handlers::common::get_assume_settings;
use handlers::common::{ load_ini, compose };
use ini::ini::Properties;
use ini::Ini;
use config::{ GetConfig };
use std::error::Error;

fn get_access_key_id(properties: &Properties) -> Option<&String> {
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

fn section_is_not_default(section: &Option<String>) -> bool {
    match section {
        Some(name) => name.to_lowercase() != "default",
        None => false
    }
}

fn find_current_assume_profile<'a>(file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    find_profile_with_name(&file, &String::from("default"))
        .and_then(compose(get_value_of_tuple, get_assume_settings))
        .and_then(find_section_with_same_assume_settings(&file))
}

fn find_current_profile<'a>(file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    find_profile_with_name(&file, &String::from("default"))
        .and_then(compose(get_value_of_tuple, get_access_key_id))
        .and_then(find_section_with_same_access_key(&file))
}

fn get_section_name((section_name, _): (&Option<String>, &Properties)) -> Option<String> {
    match section_name {
        Some(name) => Some(name.to_string()),
        None => None
    }
}

pub fn handle(config: GetConfig, mut output: impl FnMut(String) -> ()) -> Result<(), Box<Error>> {
    let config_file = load_ini(&config.config_path)?;
    let credentials_file = load_ini(&config.credentials_path)?;

    let section_name = find_current_assume_profile(&config_file)
        .or_else(|| find_current_profile(&credentials_file))
        .and_then(get_section_name);

    match section_name {
        Some(name) => Ok(output(name)),
        None => Err(String::from("no default profile set").into())
    }
}

#[cfg(test)]
mod tests {
    use ini::ini::Properties;

    mod find_section_with_same_assume_settings {
        use handlers::get;
        use ini::Ini;

        fn get_test_ini() -> Ini {
            let mut conf = Ini::new();
            conf.with_section(Some("fist_section".to_string()))
                .set("role_arn", "arn_1")
                .set("source_profile", "source_profile_1");
            conf.with_section(Some("second_section".to_string()))
                .set("role_arn", "arn_2")
                .set("source_profile", "source_profile_2");
            conf
        }

        #[test]
        fn return_none_if_not_found() {
            let conf = get_test_ini();
            let result = get::find_section_with_same_assume_settings(&conf)((&"arn_3".to_string(), &"source_profile_3".to_string()));
            assert!(result.is_none());
        }

        #[test]
        fn return_some_if_found() {
            let conf = get_test_ini();
            let result = get::find_section_with_same_assume_settings(&conf)((&"arn_2".to_string(), &"source_profile_2".to_string()));
            assert!(result.is_some());
            super::assert_section_name(result, "second_section");
        }
    }

    mod get_assume_settings {
        use handlers::get::get_assume_settings;
        use std::collections::HashMap;

        #[test]
        fn return_some_if_both_role_arn_and_source_profile_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("role_arn".to_string(), "role_arn_1".to_string());
            section_properties.insert("source_profile".to_string(), "source_profile_1".to_string());
            let result = get_assume_settings(&section_properties);

            assert!(result.is_some());
            let (arn, profile) = result.unwrap();
            assert_eq!("role_arn_1", arn);
            assert_eq!("source_profile_1", profile);
        }

        #[test]
        fn return_none_if_role_arn_not_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("source_profile".to_string(), "source_profile_1".to_string());
            let result = get_assume_settings(&section_properties);

            assert!(result.is_none());
        }

        #[test]
        fn return_none_if_source_profile_not_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("role_arn".to_string(), "role_arn_1".to_string());
            let result = get_assume_settings(&section_properties);

            assert!(result.is_none());
        }
    }

    mod find_section_with_same_access_key {
        use handlers::get::find_section_with_same_access_key;
        use ini::Ini;

        fn get_test_ini() -> Ini {
            let mut conf = Ini::new();
            conf.with_section(Some("fist_section".to_string()))
                .set("aws_access_key_id", "access_key_1");
            conf.with_section(Some("second_section".to_string()))
                .set("aws_access_key_id", "access_key_2");
            conf
        }

        #[test]
        fn return_none_if_not_found() {
            let conf = get_test_ini();
            let result = find_section_with_same_access_key(&conf)(&"access_key_3".to_string());
            assert!(result.is_none());
        }

        #[test]
        fn return_some_if_found() {
            let conf = get_test_ini();
            let result = find_section_with_same_access_key(&conf)(&"access_key_2".to_string());
            assert!(result.is_some());
            super::assert_section_name(result, "second_section");
        }
    }

    fn assert_section_name(result: Option<(&Option<String>, &Properties)>, expected: &str) {
        let (section_name, _) = result.unwrap();
        assert_eq!(section_name.clone().unwrap(), expected.to_string());
    }
}
