#![allow(dead_code)]

use file_utils::read_arg_file;

#[derive(Debug)]
struct Block {
    start: usize,
    end: usize,
    size: usize,
    id: i64
}

impl Default for Block {
    fn default() -> Block {
        Block {
            start: 0,
            end: 0,
            size: 0,
            id: -1
        }
    }
}

fn find_fist_byte_from_back(disk: &Vec<i64>) -> i64 {
    for i in 0..disk.len() {
        let x = disk.len() - i-1;
        if disk[x] >= 0 {
            return x as i64;
        }
    }

    return -1;
}

fn find_free_block_with_min_size(disk: &Vec<i64>, size: usize) -> Option<Block> {
    let mut block = Block::default();

    let mut i = 0;

    'outer: while i < disk.len()-1 {
        if disk[i] == -1 {
            block.start = i;
            for x in i+1..disk.len() {
                if disk[x] >= 0 {
                    block.end = x - 1;
                    block.size = x - i;
                    block.id = -1;

                    if block.size < size {
                        i = block.end + 1;
                        continue 'outer;
                    }

                    return Option::from(block);
                }
            }
        }
        i += 1;
    }

    return Option::None;
}

fn find_free_block(disk: &Vec<i64>, start_idx: usize) -> Option<Block> {
    let mut block = Block::default();

    for i in start_idx..disk.len()-1 {
        if disk[i] == -1 {
            block.start = i;
            for x in i+1..disk.len() {
                if disk[x] >= 0 {
                    block.end = x - 1;
                    block.size = x - i;
                    block.id = -1;

                    return Option::from(block);
                }
            }
        }
    }

    return Option::None;
}

fn print_disk(disk: &Vec<i64>) {
    for i in 0..disk.len() {
        if disk[i] == -1 {
            print!(".");
            continue;
        }
        print!("{}", disk[i]);
    }
    println!("");
}

fn read_block_back(start_idx: usize, disk: &Vec<i64>) -> Option<Block> {
    let mut block = Block::default();

    let mut i: i64 = start_idx as i64;
    while i >= 1 {
        if disk[i as usize] >= 0 {
            let mut x: i64 = i-1;
            block.end = i as usize;
            while x >= 0 {
                if disk[x as usize] != disk[i as usize] {
                    block.start = (x + 1) as usize;
                    block.size = (i - x) as usize;
                    block.id = disk[i as usize];

                    return Option::from(block);
                }
                x -= 1;
            }
        }

        i -= 1;
    }

    return Option::None;
}

fn compute_checksum(disk: &Vec<i64>) -> i64 {
    let mut checksum: i64 = 0;

    for i in 0..disk.len() {
        if disk[i] == -1 {
            continue
        }

        checksum += i as i64 * disk[i];
    }

    return checksum;
}

fn solve_part1(disk: &Vec<i64>) {
    let mut file_ids = disk.clone();

    // compress data
    for i in 0..file_ids.len() {
        if file_ids[i] >= 0 {
            continue;
        }

        let last_byte_id = find_fist_byte_from_back(&file_ids);
        // println!("{}", last_byte_id);
        if last_byte_id == -1 {
            break;
        }

        if i as i64 >= last_byte_id {
            break;
        }

        file_ids[i] = file_ids[last_byte_id as usize];
        file_ids[last_byte_id as usize] = -1;
    }

    let checksum1: i64 = compute_checksum(&file_ids);
    println!("Checksum 1: {checksum1}");
}

fn solve_part2(file_ids: &Vec<i64>) {
    let mut disk = file_ids.clone();

    let mut back_read = disk.len()-1;

    loop {
        let next_back_block = read_block_back(back_read, &disk);
        if next_back_block.is_none() {
            break;
        }
        let next_back_block = next_back_block.unwrap();
        back_read = next_back_block.start-1;

        let free_block = find_free_block_with_min_size(&disk, next_back_block.size);
        if free_block.is_none() {
            continue;
        }
        let free_block = free_block.unwrap();

        if free_block.start > next_back_block.start {
            continue;
        }

        for x in 0..next_back_block.size {
            disk[x+free_block.start] = next_back_block.id;
            disk[x+next_back_block.start] = -1;
        }
    }

    let checksum = compute_checksum(&disk);
    println!("Checksum 2: {checksum}");
}

fn solve_part2_better(file_ids: &Vec<i64>) {
    let mut disk = file_ids.clone();

    let mut free_idx = 0;
    let mut free_blocks: Vec<Block> = Vec::new();

    loop {
        let block = find_free_block(&disk, free_idx);
        if block.is_none() {
            break;
        }
        let block = block.unwrap();

        free_idx = block.end + 1;
        free_blocks.push(block);
    }

    free_idx = 0;
    let mut back_read = disk.len()-1;

    loop {
        let next_back_block = read_block_back(back_read, &disk);
        if next_back_block.is_none() {
            break;
        }
        let next_back_block = next_back_block.unwrap();
        back_read = next_back_block.start-1;

        for i in free_idx..free_blocks.len() {
            if free_blocks[i].size >= next_back_block.size {
                for x in 0..next_back_block.size {
                    disk[x+free_blocks[i].start] = next_back_block.id;
                    disk[x+next_back_block.start] = -1;
                }

                free_blocks.remove(i);
                break;
            }
        }
        print_disk(&disk);

        if free_blocks.len() == 0 {
            break;
        }
    }

    dbg!(&free_blocks);
    print_disk(&disk);

    let checksum = compute_checksum(&disk);
    println!("Checksum 2: {checksum}");
}

fn main() {
    let contents = read_arg_file();

    let mut disk: String = String::new();
    let mut id: i64 = -1;

    let mut file_ids: Vec<i64> = Vec::new();

    // create the disk from the input
    for (i, c) in contents.chars().enumerate() {
        let mut byte: char = '.';
        if i % 2 == 0 {
            byte = 'x';
            id += 1;
        }

        for _ in 0..c.to_digit(10).unwrap() {
            disk.push(byte);
            if byte == 'x' {
                file_ids.push(id);
            }
            else {
                file_ids.push(-1);
            }
        }
    }

    // println!("{}", file_ids.len());
    // dbg!(&file_ids);

    // let b1 = read_block_back(14, &file_ids);
    // if b1.is_some() {
    //     dbg!(&b1.unwrap());
    // }

    // let b2 = find_next_free_block(0, &file_ids);
    // if b2.is_some() {
    //     dbg!(&b2.unwrap());
    // }
    // print_disk(&file_ids);

    // let b1 = find_free_block_with_min_size(&file_ids, 2);
    // if b1.is_some() {
    //     dbg!(&b1.unwrap());
    // }

    // solve_part1(&file_ids);
    solve_part2(&file_ids);
    // solve_part2_better(&file_ids);
}
