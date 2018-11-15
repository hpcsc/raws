use std::env::current_dir;

pub fn get_test_data_path(file_name: String) -> String {
    let unwrapped = current_dir().unwrap();
    let current_dir = unwrapped.display();
    format!("{}/{}/{}", current_dir, "tests/test_data", file_name)
}
