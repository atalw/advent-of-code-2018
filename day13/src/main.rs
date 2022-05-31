use core::fmt;
use std::fs;

#[derive(Debug, Clone)]
struct Tracks(Vec<Vec<Position>>, Vec<Position>);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    typ: Type,
    cart: Option<Cart>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cart(Direction, u8);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Horizontal,
    Vertical,
    CurveForward,
    CurveBackward,
    Intersection,
    // Cart(Direction, u8),
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
    b();
}

fn a() {
    // let filename = "test.txt";
    let filename = "input.txt";
    let mut tracks = Tracks::new(filename);
    // println!("{}", tracks);
    for i in 0..1500 {
        tracks.tick(false);
        // println!("{}", tracks);
        if let Some(pos) = tracks.has_crash() {
            println!("crashed at {:?}", pos);
            break
        }
    }
}

fn b() {
    // let filename = "test_2.txt";
    let filename = "input.txt";
    let mut tracks = Tracks::new(filename);
    // println!("{}", tracks);
    for i in 0..100000 {
        tracks.tick(true);
        // println!("{}", tracks);
        if let Some(pos) = tracks.is_last_cart() {
            println!("{} last pos: {:?}", i, pos);
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
                let (typ, cart) = match c {
                    '|' => (Type::Vertical, None),
                    '-' => (Type::Horizontal, None),
                    '\\' => (Type::CurveBackward, None),
                    '/' => (Type::CurveForward, None),
                    '+' => (Type::Intersection, None),
                    '^' => (Type::Vertical, Some(Cart(Direction::Up, 0))),
                    'v' => (Type::Vertical, Some(Cart(Direction::Down, 0))),
                    '>' => (Type::Horizontal, Some(Cart(Direction::Right, 0))),
                    '<' => (Type::Horizontal, Some(Cart(Direction::Left, 0))),
                    ' ' => (Type::Empty, None),
                    _ => panic!(),
                };
                let position = Position {
                    x,
                    y,
                    typ,
                    cart,
                };

                if position.typ == Type::Intersection ||
                position.typ == Type::CurveBackward ||
                position.typ == Type::CurveForward { special.push(position) }

                grid[y].push(Position {
                    x,
                    y,
                    typ,
                    cart,
                });
            }
        }
        Tracks(grid, special)
    }

    fn has_crash(&mut self) -> Option<Position> {
        for row in &mut self.0 {
            for pos in row {
                if pos.typ == Type::Crash {
                    return Some(*pos)
                } else { continue }
            }
        }
        None
    }

    fn is_last_cart(&self) -> Option<Position> {
        let mut count = 0;
        let mut position: Option<Position> = None;
        for row in &self.0 {
            for pos in row {
                if pos.cart.is_some() {
                    count += 1;
                    if count > 1 { return None }
                    position = Some(*pos);
                }
            }
        }
        position
    }


    /// Upate state by one iteration
    fn tick(&mut self, remove_crash: bool) {
        let width = self.0[0].len();
        let height = self.0.len();

        macro_rules! move_cart_up {
            ($pos: expr, $inter: expr) => {{
                let top_x = $pos.x;
                let top_y = $pos.y.wrapping_sub(1);
                if top_x >= width || top_y >= height { panic!("out of bounds") }

                if self.0[top_y][top_x].cart.is_some() {
                    if remove_crash {
                        self.0[top_y][top_x].cart = None;
                    } else {
                        self.0[top_y][top_x].typ = Type::Crash;
                    }
                } else {
                    self.0[top_y][top_x].cart = match self.0[top_y][top_x].typ {
                        Type::Vertical => $pos.cart,
                        Type::CurveBackward => Some(Cart(Direction::Left, $inter)),
                        Type::CurveForward => Some(Cart(Direction::Right, $inter)),
                        Type::Intersection => {
                            match $inter {
                                0 => Some(Cart(Direction::Left, 1)),
                                1 => Some(Cart(Direction::Up, 2)),
                                2 => Some(Cart(Direction::Right, 0)),
                                _ => panic!("wrong intersection state")
                            }
                        }
                        _ => panic!("invalid next type up {:?}", self.0[top_y][top_x])
                    };
                }

                self.0[$pos.y][$pos.x].cart = None;
            }};
        }

        macro_rules! move_cart_down {
            ($pos: expr, $inter: expr) => {{
                let bottom_x = $pos.x;
                let bottom_y = $pos.y + 1;
                if bottom_x >= width || bottom_y >= height { panic!("out of bounds") }

                if self.0[bottom_y][bottom_x].cart.is_some() {
                    if remove_crash {
                        self.0[bottom_y][bottom_x].cart = None;
                    } else {
                        self.0[bottom_y][bottom_x].typ = Type::Crash;
                    }
                } else {
                    self.0[bottom_y][bottom_x].cart = match self.0[bottom_y][bottom_x].typ {
                        Type::Vertical => $pos.cart,
                        Type::CurveBackward => Some(Cart(Direction::Right, $inter)),
                        Type::CurveForward => Some(Cart(Direction::Left, $inter)),
                        Type::Intersection => {
                            match $inter {
                                0 => Some(Cart(Direction::Right, 1)),
                                1 => Some(Cart(Direction::Down, 2)),
                                2 => Some(Cart(Direction::Left, 0)),
                                _ => panic!("wrong intersection state")
                            }
                        }
                        _ => panic!("invalid next type down")
                    };
                }
                self.0[$pos.y][$pos.x].cart = None;
            }};
        }

        macro_rules! move_cart_left {
            ($pos: expr, $inter: expr) => {{
                let left_x = $pos.x.wrapping_sub(1);
                let left_y = $pos.y;
                if left_x >= width || left_y >= height { panic!("out of bounds") }

                if self.0[left_y][left_x].cart.is_some() {
                    if remove_crash {
                        self.0[left_y][left_x].cart = None;
                    } else {
                        self.0[left_y][left_x].typ = Type::Crash;
                    }
                } else {
                    self.0[left_y][left_x].cart = match self.0[left_y][left_x].typ {
                        Type::Horizontal => $pos.cart,
                        Type::CurveBackward => Some(Cart(Direction::Up, $inter)),
                        Type::CurveForward => Some(Cart(Direction::Down, $inter)),
                        Type::Intersection => {
                            match $inter {
                                0 => Some(Cart(Direction::Down, 1)),
                                1 => Some(Cart(Direction::Left, 2)),
                                2 => Some(Cart(Direction::Up, 0)),
                                _ => panic!("wrong intersection state")
                            }
                        }
                        _ => panic!("invalid next type left")
                    };
                }

                self.0[$pos.y][$pos.x].cart = None;
            }};
        }

        macro_rules! move_cart_right {
            ($pos: expr, $inter: expr) => {{
                let right_x = $pos.x + 1;
                let right_y = $pos.y;
                if right_x >= width && right_y >= height { panic!("out of bounds") }

                if self.0[right_y][right_x].cart.is_some() {
                    if remove_crash {
                        self.0[right_y][right_x].cart = None;
                    } else {
                        self.0[right_y][right_x].typ = Type::Crash;
                    }
                } else {
                    self.0[right_y][right_x].cart = match self.0[right_y][right_x].typ {
                        Type::Horizontal => $pos.cart,
                        Type::CurveBackward => Some(Cart(Direction::Down, $inter)),
                        Type::CurveForward => Some(Cart(Direction::Up, $inter)),
                        Type::Intersection => {
                            match $inter {
                                0 => Some(Cart(Direction::Up, 1)),
                                1 => Some(Cart(Direction::Right, 2)),
                                2 => Some(Cart(Direction::Down, 0)),
                                _ => panic!("wrong intersection state")
                            }
                        }
                        _ => panic!("invalid next type right")
                    };
                }
                self.0[$pos.y][$pos.x].cart = None;
            }};
        }

        let grid = self.0.clone();
        for row in &grid {
            for pos in row {
                match pos.cart {
                    Some(Cart(Direction::Up, inter)) => move_cart_up!(pos, inter),
                    Some(Cart(Direction::Down, inter)) => move_cart_down!(pos, inter),
                    Some(Cart(Direction::Left, inter)) => move_cart_left!(pos, inter),
                    Some(Cart(Direction::Right, inter)) => move_cart_right!(pos, inter),
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
                write!(f, "{}", pos)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cart.is_some() {
            write!(f, "{}", self.cart.unwrap())?;
        } else {
            write!(f, "{}", self.typ)?;
        }
        Ok(())
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cart(Direction::Up, _) => write!(f, "^")?,
            Cart(Direction::Down, _) => write!(f, "v")?,
            Cart(Direction::Right, _) => write!(f, ">")?,
            Cart(Direction::Left, _) => write!(f, "<")?,
            _ => panic!()
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
            Type::Crash => write!(f, "X")?,
            Type::Empty => write!(f, " ")?,
            _ => panic!()
        }
        Ok(())
    }
}

