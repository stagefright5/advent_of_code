use std::fs;

const FILE_PATH_PREFIX: &str = "inputs";

pub fn read_input_file(file_name: &str) -> String {
    let file_path = format!("{FILE_PATH_PREFIX}/{file_name}");
    return fs::read_to_string(file_path).unwrap();
}
