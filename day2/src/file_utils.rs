use std::fs;
use std::env;

pub fn read_file(file_path: &str) -> Vec<String>
{
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file: {file_path}");

    let lines: Vec<String> = contents.split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    return lines;
}

pub fn read_arg_file() -> Vec<String>
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    return read_file(file_path);
}
