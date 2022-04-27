use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
// Point(x, y, idx, min_dist)
struct Point(u32, u32, Index, MinDistance, TotalDistance);

type Index = Option<usize>;
type MinDistance = u32;
type TotalDistance = u32;

fn main() {
	// let filename = "test.txt";
	let filename = "input.txt";

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let mut lines: Vec<&str> = contents.split("\n").collect();
	// remove empty string
	lines.remove(lines.len()-1);
	let mut points: Vec<Point> = Vec::new();

	for line in lines {
		let v: Vec<u32> = line
			.split(", ")
			.map(|x| x.parse::<u32>().unwrap())
			.collect();
		points.push(Point(v[0],  v[1], None, MinDistance::MAX, TotalDistance::MAX));
	}

	points.sort_by_key(|p| p.1);

	println!("{:?}", points);

	let mut grid = build_grid(points.clone());
	// a(points, &mut grid);
	b(points, &mut grid);
}

fn a(mut points: Vec<Point>, grid: &mut [Vec<Point>]) {
	let mut map: HashMap<usize, u32> = HashMap::new();

	for x in 0..grid.len() {
		for y in 0..grid[0].len() {
			let res = find_shortest_distance(&points, grid[x][y]);
			grid[x][y].2 = res.0;
			grid[x][y].3 = res.1;
			// println!("{:?}", grid[x][y]);	

			if let Some(idx) = res.0 {
				points[idx].2 = res.0;
				points[idx].3 = res.1;
				let freq = map.entry(idx).or_insert(0);
				*freq += 1;
			}
		}
	}

	let max_area = map
		.iter()
		.filter(|(&idx, _)| !is_infinite_area(points[idx], &grid))
		.max_by_key(|x| x.1)
		.unwrap()
		.1;

	println!("{}", max_area);
}

fn b(points: Vec<Point>, grid: &mut [Vec<Point>]) {

	for x in 0..grid.len() {
		for y in 0..grid[0].len() {
			let res = find_total_distance(&points, grid[x][y]);
			grid[x][y].4 = res;
		}
	}

	let region: Vec<Point> = grid
		.iter()
		.flat_map(|x| x.iter())
		.filter(|p| p.4 < 10000)
		.cloned()
		.collect();
	println!("region length: {}", region.len());
}

fn find_total_distance(points: &[Point], coord: Point) -> TotalDistance {
	let mut sum = 0;
	for p in points {
		sum += manhattan_distance(*p, coord);
	}
	sum
}

/// Check in all 4 directions from point if closest extends to boundaries
fn is_infinite_area(point: Point, grid: &[Vec<Point>]) -> bool {
	let top_left = grid[0][0];
	let top_right = grid[grid.len()-1][0];
	let bottom_left = grid[0][grid[0].len()-1];
	let _bottom_right = grid[grid.len()-1][grid[0].len()-1];

	let point_north = Point(point.0, top_left.1, None, 0, 0);
	let point_south = Point(point.0, bottom_left.1, None, 0, 0);
	let point_east = Point(top_right.0, point.1, None, 0, 0);
	let point_west = Point(top_left.0, point.1, None, 0, 0);

	let mut is_infinite = true;
	for x in point.0..=point_east.0 {
		let p = grid
			.iter()
			.flat_map(|p| p.iter())
			.find(|&&p| p.0 == x && p.1 == point.1)
			.unwrap();
		if p.2 != point.2 {
			is_infinite = false;
			break;
		}
	}

	if is_infinite { return true }

	let mut is_infinite = true;
	for x in point_west.0..point.0 {
		let p = grid
			.iter()
			.flat_map(|p| p.iter())
			.find(|&&p| p.0 == x && p.1 == point.1)
			.unwrap();
		if p.2 != point.2 {
			is_infinite = false;
			break;
		}
	}

	if is_infinite { return true }

	let mut is_infinite = true;
	for y in point.1..=point_south.1 {
		let p = grid
			.iter()
			.flat_map(|p| p.iter())
			.find(|&&p| p.0 == point.0 && p.1 == y)
			.unwrap();
		if p.2 != point.2 {
			is_infinite = false;
			break;
		}
	}

	if is_infinite { return true }

	let mut is_infinite = true;
	for y in point_north.1..point.1 {
		let p = grid
			.iter()
			.flat_map(|p| p.iter())
			.find(|&&p| p.0 == point.0 && p.1 == y)
			.unwrap();
		if p.2 != point.2 {
			is_infinite = false;
			break;
		}
	}

	if is_infinite { return true }

	false
}

/// Given a coordinate on the grid, find the point which has the shortest distance
fn find_shortest_distance(points: &[Point], coord: Point) -> (Index, MinDistance) {
	let (mut idx, mut dist) = (0, MinDistance::MAX);
	let mut did_overlap_at = MinDistance::MAX; 

	for i in 0..points.len() {
		let d = manhattan_distance(points[i], coord);
		if d < dist {
			idx = i;
			dist = d;
		} else if d == dist {
			did_overlap_at = dist;
		}
	}

	if dist == did_overlap_at {
		(None, u32::MAX)
	} else if dist < coord.3 {
		(Some(idx), dist)	
	} else {
		(coord.2, coord.3)
	}
}

fn build_grid(points: Vec<Point>) -> Vec<Vec<Point>> {
	let mut grid: Vec<Vec<Point>> = Vec::new();

	let start_x = points.iter().min_by_key(|x| x.0).unwrap().0;
	let end_x = points.iter().max_by_key(|x| x.0).unwrap().0;
	let start_y = points.iter().min_by_key(|x| x.1).unwrap().1;
	let end_y = points.iter().max_by_key(|x| x.1).unwrap().1;

	for i in start_x..=end_x {
		let mut column: Vec<Point> = Vec::new();
		for j in start_y..=end_y {
			column.push(Point(i, j, None, MinDistance::MAX, TotalDistance::MAX));	
		}

		grid.push(column);
	}

	// println!("{:?}", grid);
	grid
}

fn manhattan_distance(point1: Point, point2: Point) -> u32 {
	(i32::abs(point1.0 as i32 - point2.0 as i32) + i32::abs(point1.1 as i32 - point2.1 as i32)) as u32
}
