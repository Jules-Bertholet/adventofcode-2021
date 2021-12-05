use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const GRID_SIZE: usize = 999;

fn main() {
    let lines = BufReader::new(File::open("./input").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let vents: Vec<VentLine> = lines.filter_map(|s| s.parse::<VentLine>().ok()).collect();

    let mut p1_grid = SeafloorGrid::default();
    vents
        .iter()
        .filter(|v| VentLine::is_straight(v))
        .for_each(|v| p1_grid.add_vent_line(v));

    let p1_ans = p1_grid.num_with_excessive_vents(2);

    println!("Part One: {}", p1_ans);

    let mut p2_grid = SeafloorGrid::default();
    vents.iter().for_each(|v| p2_grid.add_vent_line(v));

    let p2_ans = p2_grid.num_with_excessive_vents(2);

    println!("Part Two: {}", p2_ans);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SeafloorGrid(Box<[[usize; GRID_SIZE]; GRID_SIZE]>);

impl Default for SeafloorGrid {
    fn default() -> Self {
        SeafloorGrid(Box::new([[0; GRID_SIZE]; GRID_SIZE]))
    }
}

impl SeafloorGrid {
    fn add_vent_line(&mut self, vent_line: &VentLine) {
        let [[x1, y1], [x2, y2]] = vent_line.0;
        if x1 == x2 {
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in min_y..=max_y {
                self.0[y][x1] += 1
            }
        } else if y1 == y2 {
            let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in min_x..=max_x {
                self.0[y1][x] += 1
            }
        } else {
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            let rev = min_y != y1;
            let x_up = (x2 > x1) ^ rev;
            let mut x = if rev { x2 } else { x1 };
            for y in min_y..=max_y {
                self.0[y][x] += 1;
                if y != max_y {
                    if x_up {
                        x += 1
                    } else {
                        x -= 1
                    }
                }
            }
        }
    }
    fn num_with_excessive_vents(&self, threshold: usize) -> usize {
        self.0.iter().flatten().filter(|&&c| c >= threshold).count()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct VentLine([[usize; 2]; 2]);

impl VentLine {
    fn is_straight(&self) -> bool {
        self.0[0][0] == self.0[1][0] || self.0[0][1] == self.0[1][1]
    }
}

impl FromStr for VentLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l_str, r_str) = s.split_once(" -> ").ok_or(())?;

        let mut vent_coords: [[usize; 2]; 2] = Default::default();
        for (coord_str, coord_arr) in [l_str, r_str].into_iter().zip(vent_coords.iter_mut()) {
            let (x, y) = coord_str.split_once(",").ok_or(())?;
            coord_arr[0] = x.parse::<usize>().map_err(|_| ())?;
            coord_arr[1] = y.parse::<usize>().map_err(|_| ())?;
        }

        Ok(VentLine(vent_coords))
    }
}
