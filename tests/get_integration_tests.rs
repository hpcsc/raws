extern crate raws;
extern crate test_utilities;

use raws::handlers::get;
use raws::config;
use test_utilities::{ get_test_data_path };
use std::error::Error;

fn execute_handle(config: config::GetConfig) -> (Result<(), Box<Error>>, String) {
    let mut output_message = String::from("");

    let result = get::handle(config, |message: String| {
        output_message = message;
    });

    (result, output_message)
}

#[test]
fn return_assumed_profile_if_matching_profile_found_in_both_credentials_and_profile() {
    let config = config::GetConfig {
        config_path: get_test_data_path("get_matching_found_in_both.config".to_string()),
        credentials_path: get_test_data_path("get_matching_found_in_both.credentials".to_string())
    };

    let (result, output_message) = execute_handle(config);

    assert!(result.is_ok());
    assert_eq!("profile second_assumed_profile", output_message);
}

#[test]
fn return_profile_from_credentials_if_profile_found_in_credentials_only() {
    let config = config::GetConfig {
        config_path: get_test_data_path("get_matching_found_in_credentials_only.config".to_string()),
        credentials_path: get_test_data_path("get_matching_found_in_credentials_only.credentials".to_string())
    };

    let (result, output_message) = execute_handle(config);

    assert!(result.is_ok());
    assert_eq!("second_profile", output_message);
}

#[test]
fn return_err_if_not_found_in_both_config_and_credentials() {
    let config = config::GetConfig {
        config_path: get_test_data_path("get_not_found_in_both.config".to_string()),
        credentials_path: get_test_data_path("get_not_found_in_both.credentials".to_string())
    };

    let (result, output_message) = execute_handle(config);

    let error_message = format!("{}", result.unwrap_err());
    assert_eq!(error_message, String::from("no default profile set"));
    assert_eq!(output_message, String::from(""));
}

#[test]
fn return_err_if_config_file_not_found() {
    let config = config::GetConfig {
        config_path: get_test_data_path("not_existing.config".to_string()),
        credentials_path: get_test_data_path("get_not_found_in_both.credentials".to_string())
    };

    let (result, output_message) = execute_handle(config);
    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("failed to load file"));
    assert!(error_message.contains("not_existing.config"));
    assert_eq!(output_message, String::from(""));
}

#[test]
fn return_err_if_credentials_file_not_found() {
    let config = config::GetConfig {
        config_path: get_test_data_path("get_not_found_in_both.config".to_string()),
        credentials_path: get_test_data_path("not_existing.credentials".to_string())
    };

    let (result, output_message) = execute_handle(config);
    let error_message = format!("{}", result.unwrap_err());
    assert!(error_message.contains("failed to load file"));
    assert!(error_message.contains("not_existing.credentials"));
    assert_eq!(output_message, String::from(""));
}
