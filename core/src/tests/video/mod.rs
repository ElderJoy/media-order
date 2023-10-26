use std::{
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
};

use file_format::FileFormat;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

use crate::video::Video;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct VideoFileName {
	pub file_name: String,
	pub year: Option<u16>,
	pub part1: String,
	pub part2: String,
}

pub fn video_parse_full_file_names_list() {
	SimpleLogger::new().init().unwrap();
	// read rows from test/all_films.tst and parse them
	// check that parsed data is equal to expected data
	let file = File::open("src/tests/video/all_films.txt").unwrap();
	let reader = BufReader::new(file);

	for line in reader.lines().flatten() {
		Video::new(PathBuf::from(line), FileFormat::MatroskaVideo).discover().unwrap();
	}
}

#[test]
fn test_parse_file_name() {
	video_parse_full_file_names_list();
}

#[test]
fn check_parse_file_name() {
	let json = std::fs::read_to_string("src/tests/video/test_set.json").unwrap();
	let test_structs: Vec<VideoFileName> = serde_json::from_str(&json).expect("JSON was not well-formatted1");

	for test_struct in test_structs {
		let mut video = Video::new(PathBuf::from(test_struct.file_name), FileFormat::MatroskaVideo);

		video.discover().unwrap();
		assert_eq!(video.year, test_struct.year);
	}
}

#[test]
fn replace_range() {
	let mut my_string = String::from("Hello, world!");

	for (i, c) in my_string.chars().enumerate() {
		if c == ',' {
			my_string.replace_range(i..=i, ";");
			break;
		}
	}

	println!("{}", my_string); // prints "Hello; world!"

	let str1 = "Edpresso is better than Rust but Rust is better than C++";
	let str2 = "Rust is boring!";
	let str3 = "I love coding";
	let str4 = "Match me!";

	// Replacing some matches
	println!("{}", str1.replace("better", "best"));
	println!("{}", str2.replace("boring", "interesting!"));
	println!("{}", str3.replace("Me", "You"));
	println!("{}", str4.replace("Find", "Match"));
}
