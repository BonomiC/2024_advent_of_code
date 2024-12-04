mod file_utils;

use regex::Regex;

fn main() {
    let mut contents = file_utils::read_arg_file();

    contents = Regex::new("\\n").unwrap().replace_all(&contents, "").to_string();

    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    // (?<=mul\()\d+,\d+(?=\))
    // mul\(\d+,\d+(?=\))
    // mul\(([0-9]|[1-9][0-9]|[1-9][0-9][0-9]),([0-9]|[1-9][0-9]|[1-9][0-9][0-9])\)
    // mul\((?<first>[0-9]|[1-9][0-9]|[1-9][0-9][0-9]),(?<second>[0-9]|[1-9][0-9]|[1-9][0-9][0-9])\)
    let re = Regex::new("mul\\((?<first>[0-9]|[1-9][0-9]|[1-9][0-9][0-9]),(?<second>[0-9]|[1-9][0-9]|[1-9][0-9][0-9])\\)").unwrap();

    let caps: Vec<regex::Captures> = re.captures_iter(&contents).collect();

    for cap in caps
    {
        let num1 = &cap["first"].parse::<u32>().unwrap();
        let num2 = &cap["second"].parse::<u32>().unwrap();

        sum += num1 * num2;
    }

    // (don't\(\).*do\(\))|don't\(\).*
    // don't\(\).*?do\(\)|don't\(\).*
    let remove_dont = Regex::new("don't\\(\\).*?do\\(\\)|don't\\(\\).*").unwrap();

    contents = remove_dont.replace_all(&contents, "").to_string();

    let caps: Vec<regex::Captures> = re.captures_iter(&contents).collect();

    for cap in caps
    {
        let num1 = &cap["first"].parse::<u32>().unwrap();
        let num2 = &cap["second"].parse::<u32>().unwrap();

        sum2 += num1 * num2;
    }

    println!("Sum is {sum}");
    println!("Sum2 is {sum2}");
}
