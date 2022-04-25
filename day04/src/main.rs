use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
	let filename = "input.txt";

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong");

	let lines: Vec<&str> = contents.split("\n").collect();
	let mut records: Vec<Record> = Vec::new();

	for line in lines {
		if line.is_empty() { continue }
		let record: Record = line.parse().or_else(|err| {
			println!("{}", line);
			Err(err)
		})?;

		records.push(record);
	}

	records.sort_by_key(|x| x.timestamp);

	// fill in ids
	let mut curr_id = 0;
	for i in 0..records.len() {
		match records[i].id {
			Some(val) => { curr_id = val; continue },
			None => {
				records[i].id = Some(curr_id);
			}
		}
	}
	// println!("{:#?}", records);

	let map = a(records.clone());
	b(map);
	Ok(())
}

fn a(records: Vec<Record>) -> HashMap<u32, Vec<Timestamp>> {
	// HashMap<id, minutes of sleep>
	let mut map: HashMap<u32, Vec<Timestamp>> = HashMap::new();

	for record in records {
		if let Some(id) = record.id {
			match record.activity {
				Activity::BeginShift => {},
				Activity::FallAsleep => { 
					map.entry(id).and_modify(|x| { x.push(record.timestamp) }).or_insert(vec![record.timestamp]);
				},
				Activity::WakeUp => {
					let minutes = map.get(&id).unwrap();
					let last_minute = minutes.last().unwrap().time.minute;
					let current_minute = record.timestamp.time.minute;

					for m in last_minute+1..current_minute {
						let time_str = format!("{}:{}", record.timestamp.time.hour, m);
						let time = Time::new(&time_str).expect("uh oh");
						let timestamp = Timestamp::new(record.timestamp.date, time).expect("oh");
						map.entry(id).and_modify(|x| { x.push(timestamp) });
					}
				}
			}
		}
	}

	let choice = map.iter().max_by_key(|x| x.1.len()).unwrap();
	// println!("choice_id {:?}", choice.0);


	let mut choice_minute_map: HashMap<u8, u32> = HashMap::new();
	
	for timestamp in choice.1 {
		let minute = timestamp.time.minute;	
		let m = choice_minute_map.entry(minute).or_insert(0);
		*m += 1;
	}

	let minute = choice_minute_map.iter().max_by_key(|x| x.1).unwrap();
	assert_ne!(*minute.1, 1);
	// println!("choice_minute: {}", minute.0);

	println!("result a: {}", choice.0 * *minute.0 as u32);
	map
}

fn b(map: HashMap<u32, Vec<Timestamp>>) {
	// Hashmap<id, (minute, freq)>
	let mut max_minute_map: HashMap<u32, (u8, u32)> = HashMap::new();

	for (id, val) in map.iter() {
		// Hashmap<minute, freq>
		let mut minute_freq: HashMap<u8, u32> = HashMap::new();

		for timestamp in val {
			let f = minute_freq.entry(timestamp.time.minute).or_insert(0);
			*f += 1;
		}

		let max_m = minute_freq.iter().max_by_key(|x| x.1).map(|(&x, &y)| (x, y)).unwrap();
		max_minute_map.insert(*id, max_m);
	}

	let m = max_minute_map.iter().max_by_key(|x| x.1.1).unwrap();
	// println!("id: {:?}", m.0);
	// println!("minute: {:?}", m.1);
	println!("result b: {}", m.0 * m.1.0 as u32);

}

#[derive(Debug, Clone, Copy)]
struct Record {
	timestamp: Timestamp,
	id: Option<u32>,
	activity: Activity
}

#[derive(Debug, Clone, Copy)]
struct Timestamp {
	date: Date,
	time: Time
}

#[derive(Debug, Clone, Copy)]
struct Date {
	year: u16,
	month: u8,
	day: u8
}

#[derive(Debug, Clone, Copy)]
struct Time {
	hour: u8,
	minute: u8
}

#[derive(Debug, Clone, Copy)]
enum Activity {
	BeginShift,
	FallAsleep,
	WakeUp,
}


impl FromStr for Record {
	type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Record, Self::Err> {
		// regex parse here
		
		lazy_static! {
			static ref RE: Regex = Regex::new(r"(?x)
			(?P<date>\d{4}-\d{2}-\d{2})
			\s
			(?P<time>\d{2}:\d{2})
			\]\s
			(?P<activity>.*)	
			").unwrap();
		}

		let caps = RE.captures(s).ok_or("unrecognized record")?;

		let activity = Activity::new(&caps["activity"])?;
		let id = match activity {
			Activity::BeginShift => Some(Activity::get_id(&caps["activity"])),
			_ => None
		};

        Ok(Record {
			timestamp: Timestamp::new_from_str(&caps["date"], &caps["time"])?,
			id,
			activity,
		})
    }
}

impl Timestamp {
	fn new_from_str(date: &str, time: &str) -> Result<Self, Box<dyn Error>> {
		Ok(Timestamp { 
			date: Date::new(date)?,
			time: Time::new(time)?
		})
	}

	fn new(date: Date, time: Time) -> Result<Self, Box<dyn Error>> {
		Ok(Timestamp { 
			date,
			time
		})
	}
}

impl Date {
	fn new(from: &str) -> Result<Self, Box<dyn Error>> {
		let tokens: Vec<&str> = from.split("-").collect();
		Ok(Date { 
			year: tokens[0].parse()?,
			month: tokens[1].parse()?,
			day: tokens[2].parse()?
		})
	}
	
}

impl Time {
	fn new(from: &str) -> Result<Self, Box<dyn Error>> {
		let tokens: Vec<&str> = from.split(":").collect();
		Ok(Time { 
			hour: tokens[0].parse()?,
			minute: tokens[1].parse()?
		})
	}
	
}

impl Activity {
	fn new(from: &str) -> Result<Self, Box<dyn Error>> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"(?x)
				(?P<word>\w+)
			").unwrap();
		}
		let caps = RE.captures(from).ok_or("unrecognized activity")?;

		match &caps["word"] {
			"Guard" => Ok(Activity::BeginShift),
			"falls" => Ok(Activity::FallAsleep),
			"wakes" => Ok(Activity::WakeUp),
			_ => panic!("activity not found")
		}
	}

	fn get_id(from: &str) -> u32 {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"(?x)
				(?P<id>\d+)
			").unwrap();
		}
		let caps = RE.captures(from).unwrap();
		caps["id"].parse().unwrap()
	}
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
		(self.date, self.time).cmp(&(other.date, other.time))
	}
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
		(self.date, self.time) == (other.date, other.time)
	}
}

impl Eq for Timestamp {}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
		(self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
	}
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
		(self.year, self.month, self.day) == (other.year, other.month, other.day)
	}
}

impl Eq for Date {}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
		(self.hour, self.minute).cmp(&(other.hour, other.minute))
	}
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
		(self.hour, self.minute) == (other.hour, other.minute)
	}
}

impl Eq for Time {}
