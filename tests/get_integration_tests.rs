extern crate raws;

use std::env::current_dir;
use raws::handlers::get;
use raws::config;

fn get_test_data_path(file_name: String) -> String {
    let unwrapped = current_dir().unwrap();
    let current_dir = unwrapped.display();
    format!("{}/{}/{}", current_dir, "tests/test_data", file_name)
}

#[test]
fn return_assumed_profile_if_matching_profile_found_in_both_credentials_and_profile() {
    let config = config::GetConfig {
        config_path: get_test_data_path("get_matching_found_in_both_config".to_owned()),
        credentials_path: get_test_data_path("get_matching_found_in_both_credentials".to_owned())
    };

    let mut output_message = String::from("");

    let result = {
        let output = |message: String| {
            output_message = message;
        };

        get::handle(config, output)
    };

    assert!(result.is_ok());
    assert_eq!("profile second_assumed_profile", output_message);
}
