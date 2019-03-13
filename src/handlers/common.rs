use ini::ini::Properties;
use ini::Ini;
use ini::ini::Error::{ Io, Parse };
use shellexpand::tilde;

pub fn find_profile_with_name<'a>(file: &'a Ini, selected_profile: &String) -> Option<(&'a Option<String>, &'a Properties)> {
    file.iter().find(|(section, _)|
        match section {
            Some(section_name) => section_name.to_lowercase() == selected_profile.to_lowercase(),
            None => false
        }
    )
}

pub fn get_assume_settings(properties: &Properties) -> Option<(&String, &String)> {
    let role_arn = properties.get("role_arn");
    let source_profile = properties.get("source_profile");
    if let (Some(arn), Some(profile)) = (role_arn, source_profile) {
        return Some((arn, profile))
    }

    None
}

pub fn get_value_of_tuple<'a>((_, properties): (&Option<String>, &'a Properties)) -> &'a Properties {
    properties
}

pub fn load_ini(path: &String) -> Result<Ini, String> {
    let expanded_path = tilde(path).to_string();

    match Ini::load_from_file(expanded_path) {
        Ok(f) => Ok(f),
        Err(Io(_)) => Err(format!("failed to load file {}", path)),
        Err(Parse(_)) => Err(format!("invalid file {}", path)),
    }
}

pub fn compose<A, B, C>(fn1: impl Fn(A) -> B, fn2: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |b| fn2(fn1(b))
}

#[cfg(test)]
mod tests {
    mod find_profile_with_name {
        use handlers::common;
        use ini::Ini;

        #[test]
        fn return_none_if_profile_not_found() {
            let mut conf = Ini::new();
            conf.with_section(Some("first_section".to_string()))
                .set("some_key", "some_value");
            conf.with_section(Some("second_section".to_string()))
                .set("some_key", "some_other_value");

            let result = common::find_profile_with_name(&conf, &String::from("default"));
            assert!(result.is_none());
        }

        #[test]
        fn return_some_if_profile_found() {
            let default_section_name = "dEfAult";
            let mut conf = Ini::new();
            conf.with_section(Some("fist_section".to_string()))
                .set("some_key", "some_value");
            conf.with_section(Some(default_section_name.to_string()))
                .set("some_key", "some_other_value");

            let result = common::find_profile_with_name(&conf, &String::from("default"));
            assert!(result.is_some());
            let (section_name, _) = result.unwrap();
            assert_eq!(section_name.clone().unwrap(), default_section_name.to_string());
        }
    }

    mod get_assume_settings {
        use ini::ini::Properties;
        use handlers::common;

        #[test]
        fn return_none_if_role_arn_not_found() {
            let mut properties = Properties::new();
            properties.insert("source_profile".to_string(), "some_profile".to_string());

            let result = common::get_assume_settings(&properties);

            assert!(result.is_none());
        }

        #[test]
        fn return_none_if_source_profile_not_found() {
            let mut properties = Properties::new();
            properties.insert("role_arn".to_string(), "some_arn".to_string());

            let result = common::get_assume_settings(&properties);

            assert!(result.is_none());

        }

        #[test]
        fn return_some_if_both_role_arn_and_source_profile_available() {
            let mut properties = Properties::new();
            properties.insert("role_arn".to_string(), "some_arn".to_string());
            properties.insert("source_profile".to_string(), "some_profile".to_string());

            let result = common::get_assume_settings(&properties);

            assert_eq!(result, Some((&"some_arn".to_string(), &"some_profile".to_string())));
        }
    }
}
