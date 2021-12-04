use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let mut lines = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let draws_string = lines.next().unwrap();
    let draws: Vec<u8> = draws_string
        .trim()
        .split(',')
        .map(|s| u8::from_str(s).unwrap())
        .collect();

    let board_lines = lines.collect::<Vec<String>>();
    let mut boards: Vec<BingoBoard> = board_lines
        .chunks_exact(6)
        .map(|s| BingoBoard::from_str(&(&s[(1..)]).join("\n")).unwrap())
        .collect();

    let mut winner: Option<BingoBoard> = None;
    let mut loser: Option<BingoBoard> = None;
    'outer: for draw in draws {
        let num_boards_left = boards.len();
        for board in &mut boards {
            board.grid.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|(num, marked)| {
                    if *num == draw {
                        *marked = true;
                    }
                })
            });

            if board.is_winning() {
                board.winning_call = Some(draw);

                if winner == None {
                    winner = Some(*board);
                }

                if num_boards_left == 1 {
                    loser = Some(*board);
                    break 'outer;
                }
            }
        }

        boards.retain(|b| b.winning_call == None);
    }

    let p1_result = winner.unwrap().score().unwrap();

    println!("Part One: {}", p1_result);

    let p2_result = loser.unwrap().score().unwrap();

    println!("Part Two: {}", p2_result);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BingoBoard {
    grid: [[(u8, bool); 5]; 5],
    winning_call: Option<u8>,
}

impl BingoBoard {
    fn is_winning(&self) -> bool {
        for i in 0..5 {
            if self.grid[i].iter().all(|(_, marked)| *marked)
                || (0..5).map(|r| self.grid[r][i]).all(|(_, marked)| marked)
            {
                return true;
            }
        }

        false
    }

    fn score(&self) -> Option<u32> {
        self.winning_call.map(|c| {
            u32::from(c)
                * self
                    .grid
                    .iter()
                    .flatten()
                    .filter_map(|&(num, marked)| if marked { None } else { Some(u32::from(num)) })
                    .sum::<u32>()
        })
    }
}

impl FromStr for BingoBoard {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: [[(u8, bool); 5]; 5] = Default::default();

        for (r, row_str) in s.split('\n').enumerate() {
            for (c, num_str) in row_str.split_whitespace().enumerate() {
                grid[r][c] = (u8::from_str(num_str)?, false);
            }
        }

        Ok(BingoBoard {
            grid,
            winning_call: None,
        })
    }
}
