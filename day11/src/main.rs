use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crossbeam_utils::thread;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<i32>>);

// (x, y, power_level)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cell(usize, usize);

fn main() {
    // a();
    b();
}

fn a() {
    let serial_num = 7689;
    let grid = Grid::new(serial_num);
    let cell = grid.find_best_grid(300);
    println!("{:?}", cell);
}

fn b() {
    let serial_num = 7689;
    let grid = Grid::new(serial_num);
    let sizes: Vec<_> = (1..=300).collect();

    let max = Arc::new(Mutex::new((0, Cell(0, 0), 0)));
    thread::scope(|s| {
        for chunk in sizes.chunks(30) {
            let grid_clone = grid.clone();
            let max = Arc::clone(&max);
            s.spawn(move |_| {
                for size in chunk {
                    let (cell, power) = grid_clone.find_best_grid(*size);
                    let mut max_lock = max.lock().unwrap();
                    if max_lock.2 < power {
                        *max_lock = (*size, cell, power);
                    }
                }
                println!("{:?}", max);
            });
        }
    }).unwrap();

    println!("actual max: {:?}", max);
}

impl Grid {
    fn new(serial_num: i32) -> Self {
        let mut powers: Vec<Vec<i32>> = vec![vec![0; HEIGHT]; WIDTH];
        for y in 0..powers.len() {
            for x in 0..powers[0].len() {
                powers[x][y] = Cell(x + 1, y + 1).power(serial_num);
            }
        }

        Grid(powers)
    }

    fn find_best_grid(&self, size: usize) -> (Cell, i32) {
        // <top-left of nxn grid, total power>
        let mut map: HashMap<Cell, i32> = HashMap::new();

        for _ in 0..self.0.chunks(10).count() {
            for y in 0..self.0.len() {
                for x in 0..self.0[0].len() {
                    match self.calc_nxn_power(size, x, y) {
                        Some(p) => { 
                            map.insert(Cell(x + 1, y + 1), p); 
                        }
                        None => continue
                    }
                }
            }
        }

        let max = map.iter().max_by_key(|x| x.1).map(|(&c, &p)| (c, p)).unwrap_or((Cell(0, 0), 0));
        // println!("max power: {:?}, size: {}", max.1, size);
        max
    }

    /// Given the top left cell, find the optional 3x3 grid and return total power
    fn calc_nxn_power(&self, size: usize, x: usize, y: usize) -> Option<i32> {
        let min_x = x;
        let max_x = x + size;
        let min_y = y;
        let max_y = y + size;

        if max_x >= WIDTH || max_y >= HEIGHT { return None }

        let mut power = 0;
        for x in min_x..max_x {
            for y in min_y..max_y {
                match self.get_power(x, y) {
                    Some(p) => power += p,
                    None => continue
                }
            }
        }

        Some(power)
    }

    fn get_power(&self, x: usize, y: usize) -> Option<i32> {
        if x >= 0 && x < WIDTH && y > 0 && y <= HEIGHT {
            Some(self.0[x][y])
        } else {
            None
        }
    }
}

impl Cell {
    fn power(&self, serial_num: i32) -> i32 {
        let rack_id = self.0 + 10;
        let mut power: i32 = rack_id as i32 * self.1 as i32;
        power += serial_num;
        power *= rack_id as i32;
        power = (power / 100) % 10;
        power -= 5;
        power
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn best_grid_with_size() {
        let test_vectors = [
            (18, Cell(33, 45), 3),
            (42, Cell(21, 61), 3),
            (18, Cell(90, 269), 16),
            (42, Cell(232, 251), 12),
        ];

        for v in test_vectors {
            let grid = Grid::new(v.0);
            assert_eq!(grid.find_best_grid(v.2).0, v.1)
        }
    }

    #[test]
    fn power_calc() {
        // (cell, grid serial number, power)
        let test_vectors = [
            (Cell(3, 5), 8, 4),
            (Cell(122, 79), 57, -5),
            (Cell(217, 196), 39, 0),
            (Cell(101, 153), 71, 4),
        ];

        for v in test_vectors {
            println!("{:?}", v);
            assert_eq!(v.0.power(v.1), v.2);
        }
    }
}
