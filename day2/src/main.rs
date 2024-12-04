mod file_utils;
use file_utils::read_arg_file;

fn main()
{
    let lines = read_arg_file();

    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    for line in lines
    {
        let levels: Vec<i32> = line.split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut safe = true;

        for i in 1..levels.len()-1
        {
            let diff1 = levels[i] - levels[i-1];
            let diff2 = levels[i+1] - levels[i];

            if diff1.abs() < 1 || diff1.abs() > 3
            {
                safe = false;
                break;
            }

            if diff2.abs() < 1 || diff2.abs() > 3
            {
                safe = false;
                break;
            }

            if diff1 * diff2 < 0
            {
                safe = false;
                break;
            }
        }

        if safe
        {
            sum += 1;
            continue;
        }

        for i in 0..levels.len()
        {
            let mut levels2 = levels.clone();
            levels2.remove(i);

            safe = true;

            for i in 1..levels2.len()-1
            {
                let diff1 = levels2[i] - levels2[i-1];
                let diff2 = levels2[i+1] - levels2[i];

                if diff1.abs() < 1 || diff1.abs() > 3
                {
                    safe = false;
                    break;
                }

                if diff2.abs() < 1 || diff2.abs() > 3
                {
                    safe = false;
                    break;
                }

                if diff1 * diff2 < 0
                {
                    safe = false;
                    break;
                }
            }

            if safe {
                sum2 += 1;
                println!("{}", line);
                break;
            }
        }
    }

    sum2 += sum;

    println!("Sum is {sum}");
    println!("Sum2 is {sum2}");
}
