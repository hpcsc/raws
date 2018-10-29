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
    if let (Some(arn), Some(profile)) = (role_arn, source_profile) {
        return Some((arn, profile))
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
        Some(name) => name.to_lowercase() != "default",
        None => false
    }
}

fn find_default_section<'a>(file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    file.iter().find(|(section, _)|
        match section {
            Some(n) => n.to_lowercase() == "default",
            None => false,
        })
}

fn find_current_assume_profile<'a>(file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    find_default_section(&file)
        .and_then(compose(get_value_of_tuple, get_assume_settings))
        .and_then(find_section_with_same_assume_settings(&file))
}

fn find_current_profile<'a>(file: &'a Ini) -> Option<(&'a Option<String>, &Properties)> {
    find_default_section(&file)
        .and_then(compose(get_value_of_tuple, get_access_key_id))
        .and_then(find_section_with_same_access_key(&file))
}

fn get_section_name((section_name, _): (&Option<String>, &Properties)) -> Option<String> {
    match section_name {
        Some(name) => Some(name.to_owned()),
        None => None
    }
}

pub fn handle(config: GetConfig, output: impl Fn(String) -> ()) -> Result<(), String> {
    let config_file = load_ini(&config.config_path)?;
    let credentials_file = load_ini(&config.credentials_path)?;

    let section_name = find_current_assume_profile(&config_file)
        .or_else(|| find_current_profile(&credentials_file))
        .and_then(get_section_name);

    match section_name {
        Some(name) => Ok(output(name)),
        None => Err(String::from("no default profile set"))
    }
}

#[cfg(test)]
mod tests {
    use ini::ini::Properties;

    mod find_default_section {
        use handlers::get;
        use ini::Ini;

        #[test]
        fn return_none_if_no_default_section() {
            let mut conf = Ini::new();
            conf.with_section(Some("first_section".to_owned()))
                .set("some_key", "some_value");
            conf.with_section(Some("second_section".to_owned()))
                .set("some_key", "some_other_value");

            let result = get::find_default_section(&conf);
            assert!(result.is_none());
        }

        #[test]
        fn return_some_if_default_section_found() {
            let default_section_name = "dEfAult";
            let mut conf = Ini::new();
            conf.with_section(Some("fist_section".to_owned()))
                .set("some_key", "some_value");
            conf.with_section(Some(default_section_name.to_owned()))
                .set("some_key", "some_other_value");

            let result = get::find_default_section(&conf);
            assert!(result.is_some());
            super::assert_section_name(result, default_section_name);
        }
    }

    mod find_section_with_same_assume_settings {
        use handlers::get;
        use ini::Ini;

        fn get_test_ini() -> Ini {
            let mut conf = Ini::new();
            conf.with_section(Some("fist_section".to_owned()))
                .set("role_arn", "arn_1")
                .set("source_profile", "source_profile_1");
            conf.with_section(Some("second_section".to_owned()))
                .set("role_arn", "arn_2")
                .set("source_profile", "source_profile_2");
            conf
        }

        #[test]
        fn return_none_if_not_found() {
            let conf = get_test_ini();
            let result = get::find_section_with_same_assume_settings(&conf)((&"arn_3".to_owned(), &"source_profile_3".to_owned()));
            assert!(result.is_none());
        }

        #[test]
        fn return_some_if_found() {
            let conf = get_test_ini();
            let result = get::find_section_with_same_assume_settings(&conf)((&"arn_2".to_owned(), &"source_profile_2".to_owned()));
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
            section_properties.insert("role_arn".to_owned(), "role_arn_1".to_owned());
            section_properties.insert("source_profile".to_owned(), "source_profile_1".to_owned());
            let result = get_assume_settings(&section_properties);

            assert!(result.is_some());
            let (arn, profile) = result.unwrap();
            assert_eq!("role_arn_1", arn);
            assert_eq!("source_profile_1", profile);
        }

        #[test]
        fn return_none_if_role_arn_not_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("source_profile".to_owned(), "source_profile_1".to_owned());
            let result = get_assume_settings(&section_properties);

            assert!(result.is_none());
        }

        #[test]
        fn return_none_if_source_profile_not_available() {
            let mut section_properties = HashMap::new();
            section_properties.insert("role_arn".to_owned(), "role_arn_1".to_owned());
            let result = get_assume_settings(&section_properties);

            assert!(result.is_none());
        }
    }

    mod find_section_with_same_access_key {
        use handlers::get::find_section_with_same_access_key;
        use ini::Ini;

        fn get_test_ini() -> Ini {
            let mut conf = Ini::new();
            conf.with_section(Some("fist_section".to_owned()))
                .set("aws_access_key_id", "access_key_1");
            conf.with_section(Some("second_section".to_owned()))
                .set("aws_access_key_id", "access_key_2");
            conf
        }

        #[test]
        fn return_none_if_not_found() {
            let conf = get_test_ini();
            let result = find_section_with_same_access_key(&conf)(&"access_key_3".to_owned());
            assert!(result.is_none());
        }

        #[test]
        fn return_some_if_found() {
            let conf = get_test_ini();
            let result = find_section_with_same_access_key(&conf)(&"access_key_2".to_owned());
            assert!(result.is_some());
            super::assert_section_name(result, "second_section");
        }
    }

    fn assert_section_name(result: Option<(&Option<String>, &Properties)>, expected: &str) {
        let (section_name, _) = result.unwrap();
        assert_eq!(section_name.clone().unwrap(), expected.to_owned());
    }
}
