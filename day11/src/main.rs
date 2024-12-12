use std::mem;
use file_utils::read_arg_file;

fn main() {
    let input = read_arg_file();

    let stones: Vec<u64> = input.split(" ").map(|s| {
        s.trim().parse::<u64>().unwrap()
    }).collect();

    // solve_part1(&stones, 1);
    // solve_part1(&stones, 75);

    dbg!(blink(&blink(&stones)));
}

fn solve_part1(stones_ori: &Vec<u64>, blinks: u32) {
    let mut stones = stones_ori.clone();
    // let mut new_stones: Vec<u64> = Vec::with_capacity(stones.len());

    // for i in 0..blinks {
    //     let mut new_stones = Vec::with_capacity(stones.len());
    //     for s in 0..stones.len() {
    //         if apply_rule1(&stones, &mut new_stones, s) {continue}
    //         if apply_rule2(&stones, &mut new_stones, s) {continue}
    //         if apply_rule3(&stones, &mut new_stones, s) {continue}
    //     }
    //     // mem::swap(&mut stones, &mut new_stones);
    //     println!("After blink {} - {}", i+1, stones.len());
    //     // stones = new_stones.clone();
    // }

    let mut final_stones = Vec::<u64>::new();
    for s in 0..stones.len() {
        let mut new_stones = Vec::<u64>::new();
        new_stones.push(stones[s]);

        for i in 0..blinks {
            new_stones = blink(&new_stones);
        }

        // final_stones.append(&mut new_stones);
        mem::swap(&mut stones, &mut new_stones);
        final_stones.append(&mut new_stones);
        dbg!(&final_stones);
    }

    dbg!(&final_stones);

    println!("{} stones!", final_stones.len());
}

fn blink(stones: &Vec<u64>, new_stones: &mut Vec<u64>, max_blinks: u64, curr_blink: u64) -> Vec<u64> {
    for s in 0..stones.len() {
        if apply_rule1(stones[s], &mut new_stones) {continue}
        if apply_rule2(stones[s], &mut new_stones) {continue}
        if apply_rule3(stones[s], &mut new_stones) {continue}
    }

    // if curr_blink >= max_blinks {
    //     return new_stones.clone();
    // }
    // else {
    //     return blink(&new_stones, &mut new_stones, max_blinks, curr_blink+1);
    // }
    return Vec::new();
}

// If the stone is engraved with the number 0,
// it is replaced by a stone engraved with the number 1.
fn apply_rule1(stone: u64, new_stones: &mut Vec<u64>) -> bool {
    if stone == 0 {
        new_stones.push(1);
        return true;
    }

    return false;
}

// If the stone is engraved with a number that has
// an even number of digits, it is replaced by two stones.
// The left half of the digits are engraved on the new left stone,
// and the right half of the digits are engraved on the new right stone.
// (The new numbers don't keep extra leading zeroes:
// 1000 would become stones 10 and 0.)
fn apply_rule2(stone: u64, new_stones: &mut Vec<u64>) -> bool {
    let stone_str = stone.to_string();

    if stone_str.len() % 2 != 0 {
        return false;
    }

    let left_stone = stone_str[..stone_str.len()/2].parse::<u64>().unwrap();
    let right_stone = stone_str[stone_str.len()/2..].parse::<u64>().unwrap();

    // println!("Left {left_stone} - Right {right_stone}");

    new_stones.push(left_stone);
    new_stones.push(right_stone);

    return true;
}

// If none of the other rules apply, the stone is replaced by a new stone;
// the old stone's number multiplied by 2024 is engraved on the new stone.
fn apply_rule3(stone: u64, new_stones: &mut Vec<u64>) -> bool {
    new_stones.push(stone * 2024);

    return true;
}
