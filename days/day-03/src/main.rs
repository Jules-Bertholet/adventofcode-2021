use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let diag_nums: Vec<u16> = lines
        .filter_map(|l| u16::from_str_radix(&l, 2).ok())
        .collect();
    let num_diags = diag_nums.len();
    let gamma_rate: u16 = (0..12u16)
        .map(|s: u16| {
            u16::min(
                ((diag_nums.iter().map(|&n| (n >> s) & 1_u16).sum::<u16>() * 2u16) as usize
                    / num_diags) as u16,
                1,
            ) << s
        })
        .sum();
    let epsilon_rate = !gamma_rate & 0b0000_1111_1111_1111_u16;
    let p1_result = u32::from(gamma_rate) * u32::from(epsilon_rate);
    println!("Part One: {}", p1_result);

    let mut o_gen_candidates: Vec<u16> = diag_nums.clone();
    let mut o_gen_rating: u16 = u16::MAX; // This is an invalid value, so we can check against it
    for s in (0..12u16).rev() {
        let mask = u16::min(
            ((usize::from(
                o_gen_candidates
                    .iter()
                    .map(|&cand| (cand >> s) & 1)
                    .sum::<u16>(),
            ) * 2)
                / o_gen_candidates.len()) as u16,
            1,
        ) << s;

        o_gen_candidates.retain(|&cand| (cand & (1 << s)) == mask);
        if o_gen_candidates.len() == 1 {
            o_gen_rating = o_gen_candidates[0];
            break;
        }
    }

    if o_gen_rating == u16::MAX {
        panic!("No oxygen generator rating found!")
    }

    let mut co2_scrub_candidates: Vec<u16> = diag_nums;
    let mut co2_scrub_rating: u16 = u16::MAX; // This is an invalid value, so we can check against it
    for s in (0..12u16).rev() {
        let mask = u16::min(
            (usize::from(
                co2_scrub_candidates
                    .iter()
                    .map(|&cand| (cand >> s) & 1)
                    .sum::<u16>(),
            ) * 2
                / co2_scrub_candidates.len()) as u16,
            1,
        ) << s;
        co2_scrub_candidates.retain(|&cand| cand & (1 << s) != mask);
        if co2_scrub_candidates.len() == 1 {
            co2_scrub_rating = co2_scrub_candidates[0];
            break;
        }
    }

    if co2_scrub_rating == u16::MAX {
        panic!("No CO2 scrubber rating rating found!")
    }

    let p2_result: u32 = u32::from(o_gen_rating) * u32::from(co2_scrub_rating);

    println!("Part Two: {}", p2_result);
}
