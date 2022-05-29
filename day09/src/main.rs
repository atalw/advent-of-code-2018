use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    test();
    a();
    b();
}

fn test() {
    let input = "9 players; last marble is worth 25 points";
    let (player_num, last_point) = parse_input(input);
    let score = play(player_num, last_point);
    println!("score test: {}", score);
}

fn a() {
    let input = "465 players; last marble is worth 71940 points";
    let (player_num, last_point) = parse_input(input);
    let score = play(player_num, last_point);
    println!("score a: {}", score);
}

fn b() {
    let input = "465 players; last marble is worth 7194000 points";
    let (player_num, last_point) = parse_input(input);
    let score = play(player_num, last_point);
    println!("score b: {}", score);
}

fn play(player_num: usize, last_point: u32) -> u32 {
    let mut circle: VecDeque<u32> = VecDeque::new();
    let mut player_scores: HashMap<usize, u32> = HashMap::new();

    circle.push_back(0);
    circle.push_back(1);

    // player 0 has already played marble 1
    let mut current_player = 1;

    for i in 2..=last_point {
        if i % 23 == 0 {
            circle.rotate_left(7);
            let marble = circle.pop_back().unwrap();
            let score = player_scores.entry(current_player).or_insert(0);
            *score += i + marble;
        } else {
            circle.rotate_right(2);
            circle.push_back(i);
        }

        // println!("[{}] {:?}", current_player, circle);

        current_player = (current_player + 1) % player_num;
    }

    *player_scores.iter().max_by_key(|x| x.1).unwrap().1
}

fn parse_input(input: &str) -> (usize, u32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<num>\d+) players; last marble is worth (?P<point>\d+) points").unwrap();
    }

    let caps = RE.captures(input).expect("unrecognized input");

    let player_num: usize = caps["num"].parse().expect("player num parse");
    let last_point: u32 = caps["point"].parse().expect("last point parse");

    (player_num, last_point)
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
