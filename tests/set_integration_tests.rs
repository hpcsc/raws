extern crate ini;
extern crate raws;
extern crate test_utilities;

use ini::Ini;
use raws::handlers::set;
use raws::config;
use test_utilities::{ get_test_data_path };
use std::error::Error;

fn execute_handle(config: config::SetConfig, chosen_profile: String) -> (Result<String, Box<Error>>, Vec<String>, Vec<Ini>) {
    let mut profiles_to_choose: Vec<String> = Vec::new();
    let mut updated_files: Vec<Ini> = vec!();

    let result = {
        let choose_profile = |profiles: Vec<String>| {
            profiles_to_choose = profiles;
            Ok(chosen_profile.clone())
        };

        let write_to_file = |file: Ini, _: &String| {
            updated_files.push(file);
            Ok(())
        };

        set::handle(config, choose_profile, write_to_file)
    };

    (result, profiles_to_choose, updated_files)
}

#[test]
fn return_err_if_config_file_not_found() {
    let config = config::SetConfig {
        config_path: get_test_data_path("not_existing.config".to_string()),
        credentials_path: get_test_data_path("set.credentials".to_string()),
        pattern: "".to_string()
    };

    let (result,  _, _) = execute_handle(config, "".to_string());

    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("failed to load file"));
    assert!(error_message.contains("not_existing.config"));
}

#[test]
fn return_err_if_credentials_file_not_found() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_string()),
        credentials_path: get_test_data_path("not_existing.credentials".to_string()),
        pattern: "".to_string()
    };

    let (result,  _, _) = execute_handle(config, "".to_string());

    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("failed to load file"));
    assert!(error_message.contains("not_existing.credentials"));
}

#[test]
fn call_fzf_with_profile_names_from_both_config_and_credentials() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_string()),
        credentials_path: get_test_data_path("set.credentials".to_string()),
        pattern: "".to_string()
    };

    let (_, profiles_to_choose, _) = execute_handle(config, "".to_string());
    let expected_profiles = vec![
        "first_profile".to_string(),
        "second_profile".to_string(),
        "profile first_assumed_profile".to_string(),
        "profile second_assumed_profile".to_string(),
    ];
    assert_eq!(profiles_to_choose, expected_profiles);
}

#[test]
fn set_config_file_default_section_if_selected_profile_can_be_found_in_config() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_string()),
        credentials_path: get_test_data_path("set.credentials".to_string()),
        pattern: "".to_string()
    };

    let (_, _, updated_files) = execute_handle(config, "profile first_assumed_profile".to_string());

    let updated_config_file = &updated_files[0];
    assert_eq!(updated_config_file.get_from(Some("default"), "role_arn"), Some("1"));
    assert_eq!(updated_config_file.get_from(Some("default"), "source_profile"), Some("1"));
}

#[test]
fn set_credentials_file_default_section_if_selected_profile_can_only_be_found_in_credentials() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_string()),
        credentials_path: get_test_data_path("set.credentials".to_string()),
        pattern: "".to_string()
    };

    let (_, _, updated_files) = execute_handle(config, "first_profile".to_string());

    assert_eq!(2, updated_files.len());

    // assert that config file default section is reset
    let updated_config_file = &updated_files[0];
    assert_eq!(updated_config_file.get_from(Some("default"), "role_arn"), Some(""));
    assert_eq!(updated_config_file.get_from(Some("default"), "source_profile"), Some(""));

    let updated_credentials_file = &updated_files[1];
    assert_eq!(updated_credentials_file.get_from(Some("default"), "aws_access_key_id"), Some("1"));
    assert_eq!(updated_credentials_file.get_from(Some("default"), "aws_secret_access_key"), Some("1"));
}

#[test]
fn return_error_result_if_profile_is_not_in_both_config_and_credentials() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_string()),
        credentials_path: get_test_data_path("set.credentials".to_string()),
        pattern: "".to_string()
    };

    let (result, _, updated_files) = execute_handle(config, "third_profile".to_string());

    assert_eq!(0, updated_files.len());
    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("profile [third_profile] not found"));
}

#[test]
fn return_early_if_select_profiles_action_is_cancelled() {
    let config = config::SetConfig {
        config_path: get_test_data_path("set.config".to_string()),
        credentials_path: get_test_data_path("set.credentials".to_string()),
        pattern: "".to_string()
    };

    // when user presses Ctrl-C during fzf selection, chosen_profile is empty string
    let (result, _, updated_files) = execute_handle(config, "".to_string());

    assert_eq!(0, updated_files.len());
    assert!(result.is_ok());
}