use core::fmt;
use std::{collections::BTreeMap, fs};


const ATTACK: i32 = 3;
const HP: i32 = 200;

#[derive(Debug, PartialEq)]
struct Board(Vec<Vec<Position>>, BTreeMap<Position, Unit>);

#[derive(Debug, PartialEq, Clone)]
struct Unit {
    is_elf: bool,
    /// Attack power
    ap: i32,
    /// Hit points
    hp: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct Position(usize, usize, Type);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Type {
    Wall,
    Open,
}

impl Board {
    fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let board_str: Vec<&str> = contents.trim_end().split('\n').collect();

        let mut grid: Vec<Vec<Position>> = Vec::new();
        let mut units: BTreeMap<Position, Unit> = BTreeMap::new();

        for (y, row) in board_str.iter().enumerate() {
            let mut grid_row: Vec<Position> = Vec::new();
            for (x, c) in row.chars().enumerate() {
                let (typ, unit) = match c {
                    '.' => (Type::Open, None),
                    '#' => (Type::Wall, None),
                    'E' => (Type::Open, Some(Unit { is_elf: true, ap: ATTACK, hp: HP })),
                    'G' => (Type::Open, Some(Unit { is_elf: false, ap: ATTACK, hp: HP})),
                    _ => panic!()
                };

                let position = Position(x, y, typ); 
                grid_row.push(position.clone());
                if let Some(u) = unit { units.insert(position, u); }
            }
            grid.push(grid_row);
        }

        Board(grid, units)
    }

    fn round(&mut self) {
        // identify all targets
        // if None, end game
        // if Some, find nearest target
        // if target is in range, attack
        // else find all paths to target and select shortest path
        let units = self.1.clone();
        // let grid = &self.0;
        for (pos, unit) in units.iter() {
            if !self.1.contains_key(pos) { continue }

            let targets = match self.get_all_targets(unit) {
                Some(map) => map,
                None => break
            };

            let (target_pos, path) = self.get_nearest_target(targets, &pos);
            if target_pos.in_attack_range(pos) {
                self.attack(*pos, target_pos);
            }
            else {
                self.move_unit(*pos, path[0]);
            }
        }

    }

    /// Sweep board and return an optional list of opponent positions
    fn get_all_targets(&self, unit: &Unit) -> Option<BTreeMap<Position, Unit>> {
        let mut units = self.1.clone();
        units.retain(|_, u| u.is_elf != unit.is_elf);
        if units.is_empty() { None }
        else { Some(units) }
    }

    /// Given a list of opponent positions, find the nearest opponent and return the path
    fn get_nearest_target(&self, opp_units: BTreeMap<Position, Unit>, position: &Position) -> (Position, Vec<Position>) {
        let mut all_paths: BTreeMap<Position, Vec<Position>> = BTreeMap::new();

        for (pos, unit) in opp_units.iter() {
            all_paths.insert(*pos, self.find_shortest_paths(*position, *pos));
        }

        all_paths.iter().min_by_key(|(pos, path)| path.len()).map(|(&pos, path)| (pos, path.to_vec())).unwrap()
    }

    /// Finds the shortest paths between two positions
    fn find_shortest_paths(&self, my_pos: Position, target: Position) -> Vec<Position> {
        // There can be multiple shortest paths
        let mut paths: Vec<Vec<Position>> = Vec::new();
        let distance = target.distance(my_pos);

        self.shortest_path_internal(&mut paths, Vec::new(), my_pos, target, distance);

        assert!(!paths.is_empty());

        // Find best path by reading order
        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.top(my_pos)) {
            return path.to_vec()
        }

        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.left(my_pos)) {
            return path.to_vec()
        }

        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.right(my_pos)) {
            return path.to_vec()
        }

        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.bottom(my_pos)) {
            return path.to_vec()
        }

        panic!()
    }

    /// Recursively find all shortest paths
    fn shortest_path_internal(&self, paths: &mut Vec<Vec<Position>>, curr_path: Vec<Position>, pos: Position, target: Position, dist: u32) -> Vec<Vec<Position>> {
        if dist == 0 { return paths.to_vec() }

        if let Some(neighbours) = self.get_possible_moves(pos) {
            for n in neighbours {
                if target.distance(n) < dist {
                    // assert_eq!(target.distance(n), dist - 1);
                    let mut p = curr_path.clone();
                    p.push(n);
                    let path = self.shortest_path_internal(&mut paths.clone(), p, n, target, dist - 1);
                    println!("here {:?}", path);
                    paths.push(path);
                }
            }
        }
        paths.to_vec()
    }

    fn is_path_reachable(&self, path: Vec<Position>) -> bool {
        !(path.iter().any(|p| p.2 == Type::Wall || self.1.contains_key(p)))
    }

    fn attack(&mut self, attacker_pos: Position, victim_pos: Position) {
        let attacker = self.1.get(&attacker_pos).unwrap().clone();
        let victim = self.1.get_mut(&victim_pos).unwrap();
        victim.hp -= attacker.ap;
        if victim.hp <= 0 {
            self.1.remove(&victim_pos);
        }
    }

    /// Actually move the unit to the specified position
    fn move_unit(&mut self, curr_pos: Position, target_pos: Position) {
        assert!(target_pos.in_attack_range(&curr_pos));
        let unit = self.1.remove(&curr_pos).unwrap();
        self.1.insert(target_pos, unit);
    }

    fn get(&self, x: usize, y: usize) -> Option<Position> {
        for row in &self.0 {
            for pos in row {
                if pos.0 == x && pos.1 == y { return Some(*pos) }
            }
        }
        None
    }

    fn get_possible_moves(&self, pos: Position) -> Option<Vec<Position>> {
        let x = pos.0;
        let y = pos.1;

        let mut positions: Vec<Position> = Vec::new();

        let top_x = x;
        let top_y = y - 1;
        match self.get(top_x, top_y) {
            Some(p) => positions.push(p),
            None => {}
        }

        let bottom_x = x;
        let bottom_y = y + 1;
        match self.get(bottom_x, bottom_y) {
            Some(p) => positions.push(p),
            None => {}
        }

        let right_x = x + 1;
        let right_y = y;
        match self.get(right_x, right_y) {
            Some(p) => positions.push(p),
            None => {}
        }

        let left_x = x - 1;
        let left_y = y;
        match self.get(left_x, left_y) {
            Some(p) => positions.push(p),
            None => {}
        }

        positions.retain(|x| x.2 != Type::Wall);

        if positions.len() == 0 { None }
        else { Some(positions) }
    }

    fn left(&self, pos: Position) -> Option<Position> {
        self.get(pos.0 - 1, pos.1)
    }

    fn right(&self, pos: Position) -> Option<Position> {
        self.get(pos.0 + 1, pos.1)
    }

    fn top(&self, pos: Position) -> Option<Position> {
        self.get(pos.0, pos.1 - 1)
    }

    fn bottom(&self, pos: Position) -> Option<Position> {
        self.get(pos.0, pos.1 + 1)
    }
}

impl Unit {

}

impl Position {
    fn in_attack_range(&self, opp_position: &Position) -> bool {
        let opp_x = opp_position.0;
        let opp_y = opp_position.1;
        if self.0 == opp_x + 1 || self.0 == opp_x - 1 || self.1 == opp_y + 1 || self.1 == opp_y - 1 {
            true
        } else {
            false
        }
    }

    /// Calculate Manhattan distance between 2 positions
    fn distance(&self, pos: Position) -> u32 {
        ((self.0 as isize - pos.1 as isize).abs() + (self.1 as isize - pos.1 as isize).abs()) as u32
    }

}

fn main() {
    println!("Hello, world!");
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for pos in row {
                match self.1.get(pos) {
                    Some(unit) => write!(f, "{}", unit)?,
                    None => write!(f, "{}", pos.2)?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_elf {
            true => write!(f, "E")?,
            false => write!(f, "G")?,
        }

        Ok(())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.2)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Type::Wall => write!(f, "#")?,
            Type::Open => write!(f, ".")?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn movement() {
        let filename = "input/test_movement_one.txt";
        let mut board = Board::new(filename);

        println!("{}", board);
        board.round();
        println!("{}", board);

        let  final_filename = "input/test_movement_one_res.txt";
        let final_board = Board::new(final_filename);

        assert_eq!(board, final_board);
    }

    fn combat() {

    }
}
