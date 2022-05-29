use std::{collections::HashMap, num::Wrapping};

use lazy_static::lazy_static;
use regex::Regex;

struct CircleIndex(usize);

fn main() {
    // let input = "9 players; last marble is worth 25 points";
    let input_a = "465 players; last marble is worth 71940 points";
    let input_b = "465 players; last marble is worth 7194000 points";
    let (player_num, last_point) = parse_input(input_b);
    let score = play(player_num, last_point);
    println!("score: {}", score);
}

fn play(player_num: u32, last_point: u32) -> u32 {
    let mut circle: Vec<u32> = Vec::new();
    let mut player_scores: HashMap<usize, u32> = HashMap::new();
    let mut current_idx = CircleIndex(0);
    let mut current_player = CircleIndex(0);

    circle.push(0);

    for i in 1..=last_point {
        let last_index = circle.len();
        let plus_one;

        if current_idx == circle.len().wrapping_sub(1) {
            plus_one = CircleIndex(0);
        } else {
            plus_one = current_idx.wrapping_add(1, last_index);
        }

        if i % 23 == 0 {
            current_idx = current_idx.wrapping_sub(7, last_index);
            let marble = circle.remove(current_idx.0);
            let score = player_scores.entry(current_player.0).or_insert(0);
            *score += i + marble;
        } else {
            current_idx = plus_one.wrapping_add(1, last_index);
            circle.insert(current_idx.0, i);
        }

        // println!("[{}] {:?}", current_player.0, circle);

        current_player = current_player.wrapping_add(1, player_num as usize - 1);
    }

    *player_scores.iter().max_by_key(|x| x.1).unwrap().1
}

fn parse_input(input: &str) -> (u32, u32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<num>\d+) players; last marble is worth (?P<point>\d+) points").unwrap();
    }

    let caps = RE.captures(input).expect("unrecognized input");

    let player_num: u32 = caps["num"].parse().expect("player num parse");
    let last_point: u32 = caps["point"].parse().expect("last point parse");

    (player_num, last_point)
}

impl CircleIndex {
    fn wrapping_add(&self, rhs: usize, max: usize) -> Self {
        let idx = self.0 + rhs;
        if idx > max {
            CircleIndex(idx - max - 1)
        } else {
            CircleIndex(idx)
        }
    }

    fn wrapping_sub(&self, rhs: usize, max: usize) -> Self {
        let idx = self.0 as i32 - rhs as i32;
        if idx < 0 {
            CircleIndex((max as i32 + idx) as usize)
        } else {
            CircleIndex(idx as usize)
        }
    }
}

impl PartialEq for CircleIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<usize> for CircleIndex {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn given_examples() {
        let test_vectors = [
            ("10 players; last marble is worth 1618 points", 8317),
            ("13 players; last marble is worth 7999 points", 146373),
            ("17 players; last marble is worth 1104 points", 2764),
            ("21 players; last marble is worth 6111 points", 54718),
            ("30 players; last marble is worth 5807 points", 37305),
            ("465 players; last marble is worth 71940 points", 384475),
        ];

        for v in test_vectors {
            let (player_num, last_point) = parse_input(v.0);
            assert_eq!(play(player_num, last_point), v.1);
            println!("passed {:?}", v)
        }
    }
}
