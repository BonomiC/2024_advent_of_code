use std::fs;
use std::env;
use std::str::FromStr;
use std::fmt::Debug;

pub fn read_file(file_path: &str) -> String
{
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file: {file_path}");

    return contents;
}

pub fn read_arg_file_by_line() -> Vec<String>
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = read_file(file_path);

    let lines: Vec<String> = contents.split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    return lines;
}

pub fn read_arg_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    return read_file(file_path);
}

pub fn read_arg_file_to_2d_vec<T: Default + Clone + FromStr>() -> Vec<Vec<T>> where <T as FromStr>::Err: Debug {
    let lines = read_arg_file_by_line();

    let rows = lines.len();
    let cols = lines[0].len();

    assert_eq!(rows, cols, "Rows & cols must be same size to read as matrix!");

    let val = T::default();

    let mut mat: Vec<Vec<T>> = vec![vec![val; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let x = lines[r].chars().nth(c).unwrap().to_string().parse::<T>().unwrap();
            mat[r][c] = x;
        }
    }

    return mat;
}
