use core::fmt;
use std::{collections::{BTreeMap, BTreeSet, HashMap}, fs, cmp};


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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position(usize, usize, Type);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
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

    fn round(&mut self, with_attack: bool) {
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

            let (target_pos, target_path) = self.get_nearest_target(targets, pos);
            if target_pos.in_attack_range(*pos) && with_attack {
                // println!("attacking you");
                // println!("{:?}", target_pos);
                self.attack(*pos, target_pos);
            }
            else if let Some(path) = target_path {
                // println!("moving you");
                self.move_unit(*pos, path[0]);
            } else {
                continue
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

    /// Given a list of opponent positions, find the nearest opponent and return the path (if it
    /// exists)
    fn get_nearest_target(&self, opp_units: BTreeMap<Position, Unit>, position: &Position) -> (Position, Option<Vec<Position>>) {
        let mut all_paths: HashMap<Position, Option<Vec<Position>>> = HashMap::new();

        let mut smallest_path_len = usize::MAX;
        for (pos, _) in opp_units.iter() {
            match self.find_shortest_path(*position, *pos) {
                Some(path) => {
                    if path.len() < smallest_path_len { smallest_path_len = path.len() }
                    all_paths.insert(*pos, Some(path));
                },
                None => {
                    smallest_path_len = 0;
                    all_paths.insert(*pos, None);
                }
            }
        }

        // if *position == Position(4, 4, Type::Open) {
        //     println!("{}", self);
        //     println!("all paths: {:?}", all_paths);
        // }

        // println!("all paths: {:#?}", all_paths);
        if smallest_path_len == 0 {
            all_paths.retain(|_, p| p.is_none());
        } else {
            all_paths.retain(|_, p| p.is_some() && p.as_ref().unwrap().len() == smallest_path_len);
        }

        if all_paths.len() == 1 {
            // all_paths.iter().next().unwrap().map(|(&pos, path)| (pos, path.clone()))
            let (&pos, path) = all_paths.iter().next().unwrap();
            return (pos, path.clone())
        }

        // let shortest_paths: Vec<(Position, Option<Vec<Position>>)> = all_paths
        //     .iter()
        //     .filter(|(_, path)| path.unwrap_or(Vec::new()).len() == smallest_path_len)
        //     .map(|(&pos, &path)| (pos, path))
        //     .collect();

        if smallest_path_len == 0 {
            if let Some((&pos, path)) = all_paths.iter().find(|(&pos, _)| Some(pos) == self.top(*position)) {
                return (pos, path.clone())
            }

            if let Some((&pos, path)) = all_paths.iter().find(|(&pos, _)| Some(pos) == self.left(*position)) {
                return (pos, path.clone())
            }

            if let Some((&pos, path)) = all_paths.iter().find(|(&pos, _)| Some(pos) == self.right(*position)) {
                return (pos, path.clone())
            }

            if let Some((&pos, path)) = all_paths.iter().find(|(&pos, _)| Some(pos) == self.bottom(*position)) {
                return (pos, path.clone())
            }

        } else {
            if let Some((&pos, path)) = all_paths.iter().find(|(_, path)| Some(path.as_ref().unwrap()[0]) == self.top(*position)) {
                return (pos, path.clone())
            }

            if let Some((&pos, path)) = all_paths.iter().find(|(_, path)| Some(path.as_ref().unwrap()[0]) == self.left(*position)) {
                return (pos, path.clone())
            }

            if let Some((&pos, path)) = all_paths.iter().find(|(_, path)| Some(path.as_ref().unwrap()[0]) == self.right(*position)) {
                return (pos, path.clone())
            }

            if let Some((&pos, path)) = all_paths.iter().find(|(_, path)| Some(path.as_ref().unwrap()[0]) == self.bottom(*position)) {
                return (pos, path.clone())
            }
        }

        // Find best path by reading order

        println!("{}", self);
        println!("{:?}", position);
        println!("{:?}", all_paths);

        panic!()
    }

    /// Finds the shortest paths between two positions
    fn find_shortest_path(&self, my_pos: Position, target: Position) -> Option<Vec<Position>> {
        // There can be multiple shortest paths
        let mut paths: Vec<Vec<Position>> = Vec::new();
        let distance = target.distance(my_pos);

        // println!("start: {:?} {:?}", my_pos, target);
        self.shortest_path_internal(&mut paths, Vec::new(), my_pos, target, distance);

        paths.retain(|path| self.is_path_reachable(path.to_vec()));

        if paths.is_empty() { return None }

        // println!("paths: {:?}", paths);

        // println!("path: {:?}", paths);

        // Find best path by reading order
        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.top(my_pos)).cloned() {
            return Some(path)
        }

        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.left(my_pos)).cloned() {
            return Some(path)
        }

        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.right(my_pos)).cloned() {
            return Some(path)
        }

        if let Some(path) = paths.iter().find(|path| Some(path[0]) == self.bottom(my_pos)).cloned() {
            return Some(path)
        }

        panic!()
    }

    /// Recursively find all shortest paths
    fn shortest_path_internal(&self, paths: &mut Vec<Vec<Position>>, curr_path: Vec<Position>, pos: Position, target: Position, dist: u32) -> Option<Vec<Position>> {
        if dist == 1 { return Some(curr_path) }

        // println!("dist: {}", dist);

        if let Some(neighbours) = self.get_possible_moves(pos) {
            // println!("{:?}", neighbours);
            for n in neighbours {
                // println!("{:?}", n);
                // println!("{:?}", target.distance(n));
                if target.distance(n) < dist {
                    let mut p = curr_path.clone();
                    p.push(n);
                    // println!("here {:?}", curr_path);
                    if let Some(path) = self.shortest_path_internal(paths, p, n, target, dist - 1) {
                        paths.push(path);
                    }
                }
            }
        }

        None
    }

    fn is_path_reachable(&self, path: Vec<Position>) -> bool {
        !(path.iter().any(|pos| pos.2 == Type::Wall || self.1.contains_key(pos)))
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
        assert!(target_pos.in_attack_range(curr_pos));
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

    /// Get valid neighbours sorted in first-reading order
    fn get_possible_moves(&self, pos: Position) -> Option<Vec<Position>> {
        let x = pos.0;
        let y = pos.1;

        let mut positions: Vec<Position> = Vec::new();

        let top_x = x;
        let top_y = y - 1;
        if let Some(p) = self.get(top_x, top_y) {
            positions.push(p);
        }

        let left_x = x - 1;
        let left_y = y;
        if let Some(p) = self.get(left_x, left_y) {
            positions.push(p);
        }

        let right_x = x + 1;
        let right_y = y;
        if let Some(p) = self.get(right_x, right_y) {
            positions.push(p);
        }

        let bottom_x = x;
        let bottom_y = y + 1;
        if let Some(p) = self.get(bottom_x, bottom_y) {
            positions.push(p);
        }

        positions.retain(|x| x.2 != Type::Wall);

        if positions.is_empty() { None } else { Some(positions) }
    }

    fn top(&self, pos: Position) -> Option<Position> {
        self.get(pos.0, pos.1 - 1)
    }

    fn left(&self, pos: Position) -> Option<Position> {
        self.get(pos.0 - 1, pos.1)
    }

    fn right(&self, pos: Position) -> Option<Position> {
        self.get(pos.0 + 1, pos.1)
    }

    fn bottom(&self, pos: Position) -> Option<Position> {
        self.get(pos.0, pos.1 + 1)
    }
}

impl Unit {

}

impl Position {
    fn in_attack_range(&self, opp_position: Position) -> bool {
        let opp_x = opp_position.0;
        let opp_y = opp_position.1;
        self.distance(opp_position) == 1
    }

    /// Calculate Manhattan distance between 2 positions
    fn distance(&self, pos: Position) -> u32 {
        ((self.0 as isize - pos.0 as isize).abs() + (self.1 as isize - pos.1 as isize).abs()) as u32
    }

}

fn main() {
    println!("Hello, world!");
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<cmp::Ordering> {
        Some((self.1, self.0).cmp(&(other.1, other.0)))
    }
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
    fn movement_one() {
        let filename = "input/test_movement_one.txt";
        let mut board = Board::new(filename);

        board.round(false);

        let final_filename = "input/test_movement_one_res.txt";
        let final_board = Board::new(final_filename);

        assert_eq!(board, final_board);
    }

    #[test]
    fn movement_two() {
        let filename = "input/test_movement_two.txt";
        let mut board = Board::new(filename);

        println!("{}", board);
        board.round(false);
        println!("{}", board);
        board.round(false);
        println!("{}", board);
        board.round(false);
        println!("{}", board);

        let final_filename = "input/test_movement_two_res.txt";
        let final_board = Board::new(final_filename);

        assert_eq!(board, final_board);
    }

    fn combat() {

    }
}
