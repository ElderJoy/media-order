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

#[test]
fn video_parse_full_file_names_list() {
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
fn check_parse_file_name() {
	let json = std::fs::read_to_string("src/tests/video/test_set.json").unwrap();
	let test_structs: Vec<VideoFileName> = serde_json::from_str(&json).expect("JSON was not well-formatted1");

	for test_struct in test_structs {
		let mut video = Video::new(PathBuf::from(test_struct.file_name), FileFormat::MatroskaVideo);

		video.discover().unwrap();
		assert_eq!(video.year, test_struct.year);
	}
}
