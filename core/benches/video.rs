use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use file_format::FileFormat;
use media_order_core::video::Video;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct VideoFileName {
	pub file_name: String,
	pub year: Option<u16>,
	pub part1: String,
	pub part2: String,
}

pub fn video_parse_full_file_names_list() {
	let file = include_str!("../src/tests/video/all_films.txt");

	for line in file.lines() {
		Video::new(PathBuf::from(line), FileFormat::MatroskaVideo).discover().unwrap();
	}
}

fn video_parse_full_file_names_list_benchmark(c: &mut Criterion) {
	c.bench_function("video file parse", |b| b.iter(video_parse_full_file_names_list));
}

criterion_group!(benches, video_parse_full_file_names_list_benchmark);
criterion_main!(benches);
