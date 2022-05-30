use std::{fs, fmt};

use lazy_static::lazy_static;
use regex::Regex;

type State = Vec<bool>;

#[derive(Debug)]
struct Config(State, Vec<Rule>, usize);

#[derive(Debug)]
struct Rule(State, bool);

fn main() {
    // let filename = "test.txt";
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let mut input: Vec<&str> = contents.trim_end().split('\n').collect();
    input.remove(1);
    let mut config = parse_input(input);
    // a(config);
    b(config)
}

fn a(mut config: Config) {
    println!("0: {}", config);
    for i in 1..=20 {
        print!("{}: ", i);
        config.pad(3);
        config.evolve();
    }

    println!("{}", config.sum());
}

fn b(mut config: Config) {
    let mut total: i64 = 0;
    let mut prev_sum: i64 = 0;
    let mut difference: i64 = 0;

    for _ in 1..=1000 {
        config.pad(3);
        config.evolve();
        let curr = config.sum() as i64;
        difference = curr - prev_sum;
        prev_sum = curr;
    }

    total = prev_sum;

    total += difference * (50_000_000_000i64 - 1000);
    println!("{}", total);
}

impl Config {
    fn evolve(&mut self) {
        let mut next_gen = self.0.clone();
        for i in 2..next_gen.len() - 2  {
            let state = self.0[i-2..=i+2].to_vec();
            match self.1.iter().find(|r| r.0 == state) {
                Some(rule) => next_gen[i] = rule.1,
                None => next_gen[i] = false
            }
        }
        self.0 = next_gen;
        // println!("{}", self);
    }

    fn pad(&mut self, len: usize) {
        let mut padding = vec![false; len];
        for i in 0..4 {
            if self.0[i] {
                padding.append(&mut self.0);
                self.0 = padding;
                // update zero_index by padding
                self.2 += len;
                break
            }
        }

        let mut padding = vec![false; len];
        for i in (self.0.len()-len..self.0.len()).rev() {
            if self.0[i] {
                self.0.append(&mut padding);
                break
            }
        }
    }

    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for (i, pot) in self.0.iter().enumerate() {
            if *pot {
                sum += i as i32 - self.2 as i32;
            }
        }
        sum
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pot in &self.0 {
            if *pot {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

macro_rules! string_to_bool {
    ($input: expr, $res: expr) => {
        if $input == '.' {
            $res = false
        } else if $input == '#' {
            $res = true
        } else {
            panic!();
        }
    };
}

macro_rules! input_to_bool {
    ($input: expr, $res: expr) => {
        for c in $input.chars() {
            let r: bool;
            string_to_bool!(c, r);
            $res.push(r);
        }
    };
}

fn parse_input(input: Vec<&str>) -> Config {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"initial state: (?P<state>.+)"
        ).unwrap();
    }

    let caps = RE.captures(input[0]).expect("unrecognized initial state");
    let state = &caps["state"];
    let mut initial_state: State = Vec::new();

    input_to_bool!(state, initial_state);
    // let mut padded_initial_state = vec![false; 200];
    // let mut end_padding = padded_initial_state.clone();
    // padded_initial_state.append(&mut initial_state);
    // padded_initial_state.append(&mut end_padding);

    // let zero_index = padded_initial_state.iter().position(|&p| p).unwrap();
    let zero_index = initial_state.iter().position(|&p| p).unwrap();

    let rules = parse_rules(input[1..].to_vec());

    Config(initial_state, rules, zero_index)
}

fn parse_rules(input: Vec<&str>) -> Vec<Rule> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<rule>.+) => (?P<N>.)"
        ).unwrap();
    }

    let mut rules: Vec<Rule> = Vec::new();
    for r in input {
        let caps = RE.captures(r).expect("unrecognized rule");
        let i = &caps["rule"];
        let n_char: char = caps["N"].chars().next().unwrap();

        let mut config: State = Vec::new();
        input_to_bool!(i, config);
        let n: bool;
        string_to_bool!(n_char, n);
        let rule = Rule(config, n);
        rules.push(rule);
    }

    rules
}
