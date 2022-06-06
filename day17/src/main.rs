use std::{fs, fmt::Display};
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

struct Grid(Vec<Vec<Cell>>);

/// Square meter of the map
#[derive(Debug, PartialEq, Eq)]
struct Cell {
    typ: CellType
}

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Sand,
    Clay,
    Spring,
    StillWater,
    FlowingWater,
}

fn main() {
    println!("Hello, world!");
}

impl Grid {
    fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let input = contents.trim_end().split('\n').collect::<Vec<&str>>();

        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(x=(?P<x>\d+), y=(?P<y_start>\d+)\.\.(?P<y_end>\d+))|(y=(?P<y>\d+), x=(?P<x_start>\d+)\.\.(?P<x_end>\d+))"
            ).unwrap();
        }

        let mut clay_positions: Vec<(usize, usize)> = Vec::new();

        for row in input {
            let caps = RE.captures(row).expect("unrecognized row");
            match caps.name("x") {
                Some(x_str) => {
                    let x: usize = x_str.as_str().parse().unwrap();
                    let y_start: usize = caps["y_start"].parse().unwrap();
                    let y_end: usize = caps["y_end"].parse().unwrap();

                    for y in y_start..=y_end {
                        clay_positions.push((x, y));
                    }
                },
                None => {
                    let y: usize = caps["y"].parse().unwrap();
                    let x_start: usize = caps["x_start"].parse().unwrap();
                    let x_end: usize = caps["x_end"].parse().unwrap();

                    for x in x_start..=x_end {
                        clay_positions.push((x, y));
                    }
                }
            }
        }

        let min_x = clay_positions.iter().min_by_key(|c| c.0).unwrap().0;
        let min_y = clay_positions.iter().min_by_key(|c| c.1).unwrap().1;
        let max_x = clay_positions.iter().max_by_key(|c| c.0).unwrap().0;
        let max_y = clay_positions.iter().max_by_key(|c| c.1).unwrap().1;

        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for y in 0..=max_y-min_y {
            cells.push(Vec::new());
            for x in 0..=max_x-min_x {
                if clay_positions.contains(&(x+min_x, y+min_y)) {
                    let clay = Cell { typ: CellType::Clay };
                    cells[y].push(clay);
                    continue;
                }

                if x == 500-min_x && y == 0 {
                    let spring = Cell { typ: CellType::Spring };
                    cells[y].push(spring);
                    continue;
                }

                let sand = Cell { typ: CellType::Sand };
                cells[y].push(sand);
            }
        }

        Grid(cells)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.typ)
    }
}

impl Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CellType::Sand => write!(f, ".")?,
            CellType::Clay => write!(f, "#")?,
            CellType::Spring => write!(f, "+")?,
            CellType::StillWater => write!(f, "~")?,
            CellType::FlowingWater => write!(f, "|")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample() {
        let filename = "test.txt";
        let grid = Grid::new(filename);

        println!("{}", grid);
    }

}
