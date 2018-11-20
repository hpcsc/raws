extern crate raws;
extern crate test_utilities;

use raws::handlers::set;
use raws::config;
use test_utilities::{ get_test_data_path };

fn execute_handle(config: config::SetConfig, chosen_profile: String) -> (Result<(), String>, String, Vec<String>) {
    let mut output_message = String::from("");
    let mut profiles_to_choose: Vec<String> = Vec::new();

    let result = {
        let output = |message: String| {
            output_message = message;
        };

        let choose_profile = |profiles: Vec<String>| {
            profiles_to_choose = profiles;
            chosen_profile.clone()
        };

        set::handle(config, output, choose_profile)
    };

    (result, output_message, profiles_to_choose)
}

#[test]
fn return_err_if_config_file_not_found() {
    let config = config::SetConfig {
        config_path: get_test_data_path("not_existing.config".to_owned()),
        credentials_path: get_test_data_path("set.credentials".to_owned()),
        pattern: "".to_owned()
    };

    let (result, output_message, _) = execute_handle(config, "".to_owned());
    let error_message = result.unwrap_err();
    assert!(error_message.contains("failed to load file"));
    assert!(error_message.contains("not_existing.config"));
    assert_eq!(output_message, String::from(""));
}

#[test]
fn return_err_if_credentials_file_not_found() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_owned()),
        credentials_path: get_test_data_path("not_existing.credentials".to_owned()),
        pattern: "".to_owned()
    };

    let (result, output_message, _) = execute_handle(config, "".to_owned());
    let error_message = result.unwrap_err();
    assert!(error_message.contains("failed to load file"));
    assert!(error_message.contains("not_existing.credentials"));
    assert_eq!(output_message, String::from(""));
}

#[test]
fn call_fzf_with_profile_names_from_both_config_and_credentials() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_owned()),
        credentials_path: get_test_data_path("set.credentials".to_owned()),
        pattern: "".to_owned()
    };

    let (_, _, profiles_to_choose) = execute_handle(config, "".to_owned());
    let expected_profiles = vec![
        "first_profile".to_owned(),
        "second_profile".to_owned(),
        "profile first_assumed_profile".to_owned(),
        "profile second_assumed_profile".to_owned(),
    ];
    assert_eq!(profiles_to_choose, expected_profiles);
}
