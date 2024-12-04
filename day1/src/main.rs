mod file_utils;
use file_utils::read_file;

use std::env;
use std::collections::HashMap;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = read_file(file_path);

    let mut lvec: Vec<u32> = vec![];
    let mut rvec: Vec<u32> = vec![];
    let mut rmap: HashMap<u32, u32> = HashMap::new();

    for line in lines
    {
        let nums: Vec<&str> = line.split("   ").map(|f| f.trim() ).collect();
        lvec.push(nums[0].parse::<u32>().unwrap());
        rvec.push(nums[1].parse::<u32>().unwrap());

        rmap.entry(rvec.last().cloned().unwrap()).and_modify(|x| *x += 1).or_insert(1);
    }

    lvec.sort();
    rvec.sort();

    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    for i in 0..lvec.len()
    {
        sum += lvec[i].abs_diff(rvec[i]);

        let sim = rmap.get(&lvec[i]).cloned().unwrap_or(0);
        sum2 += lvec[i] * sim;
    }

    println!("Sum is {sum}");
    println!("Sum2 is {sum2}");
}
