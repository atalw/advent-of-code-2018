use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
struct Stars(Vec<Point>);

#[derive(Debug, Clone)]
struct Point {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

fn main() {
    // let filename = "test.txt";
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let input: Vec<&str> = contents.trim_end().split('\n').collect();
    let stars = Stars(parse_input(input));
    simulate(stars, 100000);
}

fn simulate(mut stars: Stars, duration: u32) {
    let mut sizes: Vec<(u32, i64)> = Vec::new();
    let mut stars_copy = stars.clone();

    for time in 0..duration {
        stars_copy.evolve();
        sizes.push((time, stars_copy.bounding_box()));
    }

    let min = sizes.iter().min_by_key(|x| x.1).unwrap();

    for _ in 0..=min.0 {
        stars.evolve();
    }

    println!("After {} seconds", min.0 + 1);
    let min_x = stars.0.iter().min_by_key(|x| x.pos_x).unwrap().pos_x;
    let max_x = stars.0.iter().max_by_key(|x| x.pos_x).unwrap().pos_x;
    let min_y = stars.0.iter().min_by_key(|x| x.pos_y).unwrap().pos_y;
    let max_y = stars.0.iter().max_by_key(|x| x.pos_y).unwrap().pos_y;
    stars.display(min_x, max_x, min_y, max_y);
    println!()

}

impl Stars {
    fn evolve(&mut self) {
        for point in self.0.iter_mut() {
            point.pos_x += point.vel_x;
            point.pos_y += point.vel_y;
        }
    }

    fn display(&self, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.0.iter().any(|p| p == &(x, y)) {
                    print!("⭐️");
                } else {
                    print!("🎄");
                }
            }
            println!();
        }
    }

    fn bounding_box(&self) -> i64 {
        let min_x = self.0.iter().min_by_key(|x| x.pos_x).unwrap().pos_x;
        let max_x = self.0.iter().max_by_key(|x| x.pos_x).unwrap().pos_x;
        let min_y = self.0.iter().min_by_key(|x| x.pos_y).unwrap().pos_y;
        let max_y = self.0.iter().max_by_key(|x| x.pos_y).unwrap().pos_y;

        let x = max_x - min_x;
        let y = max_y - min_y;

        x as i64 * y as i64
    }
}

impl PartialEq<(i32, i32)> for Point {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.pos_x == other.0 && self.pos_y == other.1
    }
}

fn parse_input(input: Vec<&str>) -> Vec<Point> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            position=<\s*(?P<pos_x>\-?\d+),\s+(?P<pos_y>\-?\d+)>
            \s
            velocity=<\s*(?P<vel_x>\-?\d+),\s+(?P<vel_y>\-?\d+)>
        ").unwrap();
    }

    let mut points: Vec<Point> = Vec::new();

    for p in input {
        let caps = RE.captures(p).expect("unrecognized input");

        let pos_x: i32 = caps["pos_x"].parse().expect("pos_x parse");
        let pos_y: i32 = caps["pos_y"].parse().expect("pos_y parse");
        let vel_x: i32 = caps["vel_x"].parse().expect("vel_x parse");
        let vel_y: i32 = caps["vel_y"].parse().expect("vel_y parse");


        points.push(Point {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        });


    }

    points
}
