use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Eq, Hash, Clone)]
struct Coord(u32, u32);

#[derive(Debug, Clone)]
struct Size(u32, u32);

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.1 == other.1		
	}
}

fn main() {
	let filename = "input.txt";

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let lines: Vec<&str> = contents.split("\n").collect();
	let mut map: HashMap<u32, (Coord, Size)> = HashMap::new();

	for line in lines {
		if line.is_empty() { continue }
		let v: Vec<&str> = line.split(" ").collect();

		let id: u32 = str::replace(v[0], "#", "").parse::<u32>().unwrap();

		let xy: Vec<u32> = str::replace(v[2], ":", "").split(",").map(|x| x.parse::<u32>().unwrap()).collect();
		let coord = Coord(xy[0], xy[1]);

		let wh: Vec<u32> = v[3].split("x").map(|x| x.parse::<u32>().unwrap()).collect();
		let size = Size(wh[0], wh[1]);

		map.insert(id, (coord, size));
	}
	
	let overlapped_fabric = a(map.clone());
	b(map, overlapped_fabric);
}

fn a(map: HashMap<u32, (Coord, Size)>) -> HashSet<Coord> {

	let mut fabric: HashSet<Coord> = HashSet::new();
	let mut overlapped_fabric: HashSet<Coord> = HashSet::new();

	for i in 0..1000 {
		for j in 0..1000 {
			let coord = Coord(i, j);
			fabric.insert(coord);
		}
	}

	for val in map.iter() {
		let coord = &val.1.0;
		let wh = &val.1.1;
		
		
		for x in coord.0..coord.0+wh.0 {
			for y in coord.1..coord.1+wh.1 {
				let c = Coord(x, y);

				match fabric.remove(&c) {
					true => {},
					false => { overlapped_fabric.insert(c); }
				}
			}
		}
	}

	println!("a: {}", overlapped_fabric.len());
	overlapped_fabric
}

fn b(map: HashMap<u32, (Coord, Size)>, overlapped_fabric: HashSet<Coord>) {

	for val in map.iter() {
		let id = &val.0;
		let coord = &val.1.0;
		let wh = &val.1.1;

		let mut did_overlap = false;

		for x in coord.0..coord.0+wh.0 {
			for y in coord.1..coord.1+wh.1 {
				let c = Coord(x, y);
				match overlapped_fabric.contains(&c) {
					true => { did_overlap = true },
					false => {}
				}
			}
		}

		if !did_overlap {
			println!("b: {}", id);
		}
	}
}
