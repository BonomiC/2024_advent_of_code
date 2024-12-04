use std::fs;

pub fn read_file(file_path: &str) -> Vec<String>
{
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file: {file_path}");

    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    if lines[lines.len()-1].len() == 0 {
        lines.truncate(lines.len()-1);
    }

    return lines;
}
