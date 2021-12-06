use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let mut lines = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let fishes: Vec<InitLanternfish> = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| InitLanternfish::from_str(s).unwrap())
        .collect();

    let mut model = LanternfishModel::new(&fishes);

    println!("Part One: {}", model.total_pop(80));

    println!("Part Two: {}", model.total_pop(256));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InitLanternfish(u8);

impl Default for InitLanternfish {
    fn default() -> Self {
        InitLanternfish(8)
    }
}

impl FromStr for InitLanternfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u8 = u8::from_str(s).map_err(|_| ())?;

        if num > 6 {
            Err(())
        } else {
            Ok(InitLanternfish(num))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LanternfishModel {
    reproducing_ages: Vec<Option<usize>>,
    num_fish: Vec<Option<usize>>,
}

impl LanternfishModel {
    fn new(init_f: &[InitLanternfish]) -> LanternfishModel {
        let mut init_arr: [usize; 7] = [0; 7];

        for fish in init_f {
            init_arr[usize::from(fish.0)] += 1;
        }

        LanternfishModel {
            reproducing_ages: init_arr.iter().map(|&u| Some(u)).collect(),
            num_fish: vec![Some(init_f.len())],
        }
    }
}

impl LanternfishModel {
    fn num_reproducing_age(&mut self, d: usize) -> usize {
        if let Some(Some(ans)) = self.reproducing_ages.get(d) {
            *ans
        } else {
            let ans = self.num_reproducing_age(d - 7)
                + (if d >= 9 {
                    self.num_reproducing_age(d - 9)
                } else {
                    0
                });
            if self.reproducing_ages.len() <= d {
                self.reproducing_ages.resize(d + 1, None);
            }
            self.reproducing_ages[d] = Some(ans);
            ans
        }
    }

    fn total_pop(&mut self, d: usize) -> usize {
        if let Some(Some(ans)) = self.num_fish.get(d) {
            *ans
        } else {
            let ans = self.total_pop(d - 1) + self.num_reproducing_age(d - 1);

            if self.num_fish.len() <= d {
                self.num_fish.resize(d + 1, None);
            }
            self.num_fish[d] = Some(ans);
            ans
        }
    }
}
