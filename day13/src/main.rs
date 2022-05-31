use core::fmt;
use std::fs;

#[derive(Debug, Clone)]
struct Tracks(Vec<Vec<Position>>, Vec<Position>);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    typ: Type,
}

struct Cart(Direction, Option<Direction>);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Horizontal,
    Vertical,
    CurveForward,
    CurveBackward,
    Intersection,
    Cart(Direction, u8),
    // CartUp,
    // CartDown,
    // CartLeft,
    // CartRight,
    Crash,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    a();
}

fn a() {
    // let filename = "test.txt";
    let filename = "input.txt";
    let mut tracks = Tracks::new(filename);
    println!("{}", tracks);
    for i in 0..1500 {
        tracks.tick();
        println!("{}", tracks);
        if let Some(pos) = tracks.has_crash() {
            println!("crashed at {:?}", pos);
            break
        }
    }
}

impl Tracks {
    fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let grid_str: Vec<&str> = contents.trim_end().split('\n').collect();

        let mut grid: Vec<Vec<Position>> = vec![Vec::new(); grid_str.len()];
        let mut special: Vec<Position> = Vec::new();

        for (y, row) in grid_str.iter().enumerate() {
            grid[y] = Vec::new();
            for (x, c) in row.chars().enumerate() {
                let typ = match c {
                    '|' => Type::Vertical,
                    '-' => Type::Horizontal,
                    '\\' => Type::CurveBackward,
                    '/' => Type::CurveForward,
                    '+' => Type::Intersection,
                    '^' => Type::Cart(Direction::Up, 0),
                    'v' => Type::Cart(Direction::Down, 0),
                    '>' => Type::Cart(Direction::Right, 0),
                    '<' => Type::Cart(Direction::Left, 0),
                    ' ' => Type::Empty,
                    _ => panic!(),
                };
                let position = Position {
                    x,
                    y,
                    typ
                };

                if position.typ == Type::Intersection ||
                position.typ == Type::CurveBackward ||
                position.typ == Type::CurveForward { special.push(position) }

                grid[y].push(Position {
                    x,
                    y,
                    typ
                });
            }
        }
        Tracks(grid, special)
    }

    fn has_crash(&self) -> Option<Position> {
        for row in &self.0 {
            for pos in row {
                if pos.typ == Type::Crash {
                    return Some(*pos)
                } else { continue }
            }
        }
        None
    }


    /// Upate state by one iteration
    fn tick(&mut self) {
        let width = self.0[0].len();
        let height = self.0.len();

        macro_rules! update_pos {
            ($pos: expr) => {
                match $pos.typ {
                    Type::Cart(Direction::Up, inter) => {
                        let top_x = $pos.x;
                        let top_y = $pos.y.wrapping_sub(1);
                        if top_x >= width || top_y >= height { panic!("out of bounds") }

                        self.0[top_y][top_x].typ = match self.0[top_y][top_x].typ {
                            Type::Cart(_, _) => Type::Crash,
                            Type::Vertical => $pos.typ,
                            Type::CurveBackward => Type::Cart(Direction::Left, inter),
                            Type::CurveForward => Type::Cart(Direction::Right, inter),
                            Type::Intersection => {
                                match inter {
                                    0 => Type::Cart(Direction::Left, 1),
                                    1 => Type::Cart(Direction::Up, 2),
                                    2 => Type::Cart(Direction::Right, 0),
                                    _ => panic!("wrong intersection state")
                                }
                            }
                            _ => panic!("invalid next type up {:?}", self.0[top_y][top_x])
                        };
                        let intersection_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::Intersection
                        };
                        let curve_forward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveForward,
                        };
                        let curve_backward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveBackward,
                        };
                        if self.1.contains(&intersection_pos) {
                            self.0[$pos.y][$pos.x] = intersection_pos;
                        } else if self.1.contains(&curve_forward_pos) {
                            self.0[$pos.y][$pos.x] = curve_forward_pos;
                        } else if self.1.contains(&curve_backward_pos) {
                            self.0[$pos.y][$pos.x] = curve_backward_pos;
                        } else {
                            self.0[$pos.y][$pos.x].typ = Type::Vertical;
                        }
                    },
                    Type::Cart(Direction::Down, inter) => {
                        let bottom_x = $pos.x;
                        let bottom_y = $pos.y + 1;
                        if bottom_x >= width || bottom_y >= height { panic!("out of bounds") }

                        self.0[bottom_y][bottom_x].typ = match self.0[bottom_y][bottom_x].typ {
                            Type::Cart(_, _) => Type::Crash,
                            Type::Vertical => $pos.typ,
                            Type::CurveBackward => Type::Cart(Direction::Right, inter),
                            Type::CurveForward => Type::Cart(Direction::Left, inter),
                            Type::Intersection => {
                                match inter {
                                    0 => Type::Cart(Direction::Right, 1),
                                    1 => Type::Cart(Direction::Down, 2),
                                    2 => Type::Cart(Direction::Left, 0),
                                    _ => panic!("wrong intersection state")
                                }
                            }
                            _ => panic!("invalid next type down")
                        };
                        let intersection_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::Intersection
                        };
                        let curve_forward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveForward,
                        };
                        let curve_backward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveBackward,
                        };
                        if self.1.contains(&intersection_pos) {
                            self.0[$pos.y][$pos.x] = intersection_pos;
                        } else if self.1.contains(&curve_forward_pos) {
                            self.0[$pos.y][$pos.x] = curve_forward_pos;
                        } else if self.1.contains(&curve_backward_pos) {
                            self.0[$pos.y][$pos.x] = curve_backward_pos;
                        } else {
                            self.0[$pos.y][$pos.x].typ = Type::Vertical;
                        }
                    },
                    Type::Cart(Direction::Left, inter) => {
                        let left_x = $pos.x.wrapping_sub(1);
                        let left_y = $pos.y;
                        if left_x >= width || left_y >= height { panic!("out of bounds") }

                        self.0[left_y][left_x].typ = match self.0[left_y][left_x].typ {
                            Type::Cart(_, _) => Type::Crash,
                            Type::Horizontal => $pos.typ,
                            Type::CurveBackward => Type::Cart(Direction::Up, inter),
                            Type::CurveForward => Type::Cart(Direction::Down, inter),
                            Type::Intersection => {
                                match inter {
                                    0 => Type::Cart(Direction::Down, 1),
                                    1 => Type::Cart(Direction::Left, 2),
                                    2 => Type::Cart(Direction::Up, 0),
                                    _ => panic!("wrong intersection state")
                                }
                            }
                            _ => panic!("invalid next type left")
                        };
                        let intersection_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::Intersection
                        };
                        let curve_forward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveForward,
                        };
                        let curve_backward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveBackward,
                        };
                        if self.1.contains(&intersection_pos) {
                            self.0[$pos.y][$pos.x] = intersection_pos;
                        } else if self.1.contains(&curve_forward_pos) {
                            self.0[$pos.y][$pos.x] = curve_forward_pos;
                        } else if self.1.contains(&curve_backward_pos) {
                            self.0[$pos.y][$pos.x] = curve_backward_pos;
                        } else {
                            self.0[$pos.y][$pos.x].typ = Type::Horizontal;
                        }
                    },
                    Type::Cart(Direction::Right, inter) => {
                        let right_x = $pos.x + 1;
                        let right_y = $pos.y;
                        if right_x >= width && right_y >= height { panic!("out of bounds") }

                        self.0[right_y][right_x].typ = match self.0[right_y][right_x].typ {
                            Type::Cart(_, _) => Type::Crash,
                            Type::Horizontal => $pos.typ,
                            Type::CurveBackward => Type::Cart(Direction::Down, inter),
                            Type::CurveForward => Type::Cart(Direction::Up, inter),
                            Type::Intersection => {
                                match inter {
                                    0 => Type::Cart(Direction::Up, 1),
                                    1 => Type::Cart(Direction::Right, 2),
                                    2 => Type::Cart(Direction::Down, 0),
                                    _ => panic!("wrong intersection state")
                                }
                            }
                            _ => panic!("invalid next type right")
                        };
                        let intersection_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::Intersection
                        };
                        let curve_forward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveForward,
                        };
                        let curve_backward_pos = Position {
                            x: $pos.x,
                            y: $pos.y,
                            typ: Type::CurveBackward,
                        };
                        if self.1.contains(&intersection_pos) {
                            self.0[$pos.y][$pos.x] = intersection_pos;
                        } else if self.1.contains(&curve_forward_pos) {
                            self.0[$pos.y][$pos.x] = curve_forward_pos;
                        } else if self.1.contains(&curve_backward_pos) {
                            self.0[$pos.y][$pos.x] = curve_backward_pos;
                        } else {
                            self.0[$pos.y][$pos.x].typ = Type::Horizontal;
                        }
                    },
                    _ => panic!("only carts can move") 
                }
            };
        }

        let grid = self.0.clone();
        for (y, row) in grid.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                match pos.typ {
                    Type::Cart(_, _) => update_pos!(pos),
                    _ => continue,
                }
            }
        }
    }

    fn get(&mut self, x: usize, y: usize) -> Option<&mut Position> {
        if x < self.0[0].len() && y < self.0.len() { Some(&mut self.0[y][x]) }
        else { None }
    }
}

impl fmt::Display for Tracks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.0.len() {
            for pos in &self.0[y] {
                write!(f, "{}", pos.typ)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Type::Horizontal => write!(f, "-")?,
            Type::Vertical => write!(f, "|")?,
            Type::CurveForward => write!(f, "/")?,
            Type::CurveBackward => write!(f, "\\")?,
            Type::Intersection => write!(f, "+")?,
            Type::Cart(Direction::Up, _) => write!(f, "^")?,
            Type::Cart(Direction::Down, _) => write!(f, "v")?,
            Type::Cart(Direction::Right, _) => write!(f, ">")?,
            Type::Cart(Direction::Left, _) => write!(f, "<")?,
            Type::Crash => write!(f, "X")?,
            Type::Empty => write!(f, " ")?,
            _ => panic!()
        }
        Ok(())
    }
}
