use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
    str::FromStr,
};

fn main() {
    let lines = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let command_strings: Vec<String> = lines.collect();

    let p1_commands = command_strings
        .iter()
        .filter_map(|l| Command::from_str(l).ok());
    let p1_final_pos = p1_commands.fold(PartOneState::default(), |acc, command| acc + command);
    let p1_result = p1_final_pos.depth * p1_final_pos.horizontal;
    println!("Part One: {}", p1_result);

    let p2_commands = command_strings
        .iter()
        .filter_map(|l| Command::from_str(l).ok());
    let p2_final_pos = p2_commands.fold(PartTwoState::default(), |acc, command| acc + command);
    let p2_result = p2_final_pos.depth * p2_final_pos.horizontal;
    println!("Part Two: {}", p2_result);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Command {
    Depth(i32, bool),
    Horizontal(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num_str) = s.strip_prefix("forward ") {
            match i32::from_str(num_str) {
                Ok(i) => Ok(Self::Horizontal(i)),
                Err(_) => Err(()),
            }
        } else if let Some(num_str) = s.strip_prefix("down ") {
            match i32::from_str(num_str) {
                Ok(i) => Ok(Self::Depth(i, true)),
                Err(_) => Err(()),
            }
        } else if let Some(num_str) = s.strip_prefix("up ") {
            match i32::from_str(num_str) {
                Ok(i) => Ok(Self::Depth(i, false)),
                Err(_) => Err(()),
            }
        } else {
            todo!()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PartOneState {
    depth: i32,
    horizontal: i32,
}

impl Add<Command> for PartOneState {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs {
            Command::Depth(d_depth, true) => Self {
                depth: self.depth + d_depth,
                horizontal: self.horizontal,
            },
            Command::Depth(d_depth, false) => Self {
                depth: self.depth - d_depth,
                horizontal: self.horizontal,
            },
            Command::Horizontal(d_horiz) => Self {
                depth: self.depth,
                horizontal: self.horizontal + d_horiz,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PartTwoState {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Add<Command> for PartTwoState {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs {
            Command::Depth(d_aim, true) => Self {
                depth: self.depth,
                horizontal: self.horizontal,
                aim: self.aim + d_aim,
            },
            Command::Depth(d_aim, false) => Self {
                depth: self.depth,
                horizontal: self.horizontal,
                aim: self.aim - d_aim,
            },
            Command::Horizontal(d_horiz) => Self {
                depth: self.depth + d_horiz * self.aim,
                horizontal: self.horizontal + d_horiz,
                aim: self.aim,
            },
        }
    }
}
