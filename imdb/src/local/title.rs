use std::io::{BufRead, BufReader};

pub struct Title {
	pub title_id: String,
	pub ordering: u8,
	pub title: String,
	pub region: Option<String>,
	pub language: Option<String>,
	pub types: Option<String>,
	pub attributes: Option<String>,
	pub is_original_title: bool,
}

pub fn parse_gzip_file(file_name: &str) -> Vec<Title> {
	let file = std::fs::File::open(file_name).unwrap();
	let reader = BufReader::new(file);
	let gzip = flate2::read::GzDecoder::new(reader);
	// let mut buffer = String::new();

	// gzip.read_to_string(&mut buffer).unwrap();

	let mut titles = Vec::new();
	for line in BufReader::new(gzip).lines().flatten() {
		println!("{}", line);
		titles.push(parse_title(&line));
	}

	titles
}

pub fn parse_title(line: &str) -> Title {
	let mut parts = line.split("\t");

	Title {
		title_id: parts.next().unwrap().to_string(),
		ordering: parts.next().unwrap().parse().unwrap(),
		title: parts.next().unwrap().to_string(),
		region: parts.next().map(|s| s.to_string()),
		language: parts.next().map(|s| s.to_string()),
		types: parts.next().map(|s| s.to_string()),
		attributes: parts.next().map(|s| s.to_string()),
		is_original_title: parts.next().unwrap() == "1",
	}
}
