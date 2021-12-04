use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let lines = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let depths: Vec<u32> = lines.filter_map(|l| u32::from_str(&l).ok()).collect();

    let part_1_result = depths
        .windows(2)
        .filter(|&slice| slice[1] > slice[0])
        .count();

    println!("Part One: {}", part_1_result);

    let window_sums: Vec<u32> = depths.windows(3).map(|slice| slice.iter().sum()).collect();

    let part_2_result = window_sums
        .windows(2)
        .filter(|&slice| slice[1] > slice[0])
        .count();

    println!("Part Two: {}", part_2_result);
}
