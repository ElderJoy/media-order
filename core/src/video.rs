extern crate ffmpeg_the_third as ffmpeg;

use std::path::{Path, PathBuf};

use chrono::{Datelike, Local};
use ffmpeg::format::context::Input as ffmpegContext;
use file_format::FileFormat;
use log::debug;

use crate::errors::{MediaOrderError, Result};

#[derive(Clone, Debug)]
pub struct Lang {
	name: &'static str,
	code: &'static str,
}

fn check_lang(str: &str, langs: &Vec<Lang>) -> Option<Lang> {
	let str = str.to_lowercase();
	for lang in langs {
		if str == lang.name || str == lang.code {
			return Some(lang.clone());
		}
	}
	None
}

lazy_static! {
	static ref LANG2: Vec<Lang> = vec![
		Lang { name: "eng", code: "en" },
		Lang { name: "rus", code: "ru" },
		Lang { name: "ukr", code: "ua" },
		Lang { name: "fra", code: "fr" },
	];
}

lazy_static! {
	static ref LANG: Vec<&'static str> = vec!["en", "eng", "ru", "rus", "ua", "ukr", "fr", "fra"];
}

lazy_static! {
	static ref VENC: Vec<&'static str> = vec!["xvid", "divx", "avc", "x264", "x265", "h264", "h265", "hevc"];
}

lazy_static! {
	static ref AENC: Vec<&'static str> = vec!["ac3", "dts", "aac", "mp3", "flac", "opus"];
}

lazy_static! {
	static ref VRES: Vec<&'static str> =
		vec!["2160p", "1080p", "1080", "720p", "480p", "360p", "240p", "144p"];
}

lazy_static! {
	static ref VQUAL: Vec<&'static str> = vec![
		"dvdrip",
		"bdrip",
		"hdrip",
		"hdtvrip",
		"hdts-rip",
		"tvrip",
		"web-dlrip",
		"webrip",
		"web-dl",
		"remux"
	];
}

lazy_static! {
	static ref TRASH: Vec<Vec<&'static str>> = vec![
		vec!["hdclub"],
		vec!["hqclub"],
		vec!["elektri4ka"],
		vec!["interfilm"],
		vec!["releaseavcgroup"],
		vec!["torrents", "ru"],
		vec!["rutracker", "org"],
		vec!["scarabey", "org"],
		vec!["nolimits-team"],
		vec!["uniongang", "ru"],
		vec!["www", "kinokopilka", "ru"],
		vec!["by", "anvic", "www", "fenixclub", "com"],
		vec!["by", "scarabey"],
		vec!["by", "dalemake"],
		vec!["freerutor"],
		vec!["kinozal", "tv"],
		vec!["hellywood"],
		vec!["truavc"],
		vec!["serbin"],
		vec!["eniahd"],
		vec!["tfile", "ru"],
		vec!["Goblin"],
	];
}

lazy_static! {
	static ref DASH_TRASH: Vec<&'static str> =
		vec!["-vaippp", "-hqclub", "-hq-video", "-mediaclub", "-kyle",];
}

fn remove_trash(parts: &mut Vec<&str>) {
	for i in 0..parts.len() {
		if i >= parts.len() {
			break;
		}

		if parts[i].starts_with('-') && DASH_TRASH.contains(&parts[i].to_lowercase().as_str()) {
			parts.remove(i);
			continue;
		}

		for trash in TRASH.iter() {
			if parts.len() < i + trash.len() {
				continue;
			}
			for j in 0..trash.len() {
				if parts[i + j].to_lowercase() != trash[j] {
					break;
				}
				if j == trash.len() - 1 {
					parts.drain(i..i + trash.len());
				}
			}
		}
	}
}

lazy_static! {
	static ref VIDEO_EXTENSIONS: Vec<&'static str> = vec!["avi", "mkv", "mp4", "m4v", "mov"];
}

lazy_static! {
	static ref SEPARATORS: Vec<char> = vec![' ', '.', '_'];
}

lazy_static! {
	static ref BRACKETS: Vec<char> = vec!['(', ')', '[', ']'];
}

lazy_static! {
	static ref CUR_YEAR: u16 = Local::now().year() as u16;
}

pub struct Video {
	pub path: PathBuf,
	pub format: FileFormat,
	pub name_original: String,
	pub name_english: Option<String>,
	pub ffmpeg_context: Option<ffmpegContext>,
	pub year: Option<u16>,
	pub genre: Option<String>,
	pub lang: Option<Vec<Lang>>,
	pub ext: Option<String>,
	pub venc: Option<String>,
	pub aenc: Option<String>,
	pub vres: Option<String>,
	pub vqual: Option<String>,
}

impl Video {
	pub fn init() {
		ffmpeg::init().unwrap();
	}

	pub fn new(path: PathBuf, format: FileFormat) -> Self {
		Self {
			path: path.clone(),
			format,
			name_original: String::new(),
			ffmpeg_context: None,
			name_english: None,
			year: None,
			genre: None,
			lang: None,
			ext: None,
			venc: None,
			aenc: None,
			vres: None,
			vqual: None,
		}
	}

	pub fn read_ffmpeg_content(&mut self) -> Result<()> {
		self.ffmpeg_context =
			Some(ffmpeg::format::input(&self.path).map_err(|_| MediaOrderError::VideoMetadata)?);
		debug!("{}", self.get_full_ffmpeg_context());
		Ok(())
	}

	pub fn discover(&mut self) -> Result<()> {
		self.parse_file_name()
	}

	fn _check_set_year(&mut self, file_name: &str) -> Option<usize> {
		let mut year = 0;
		let mut digit_count = 0;
		let mut look_separator = true;

		for (i, c) in file_name.chars().rev().enumerate() {
			if SEPARATORS.contains(&c) {
				if digit_count == 4 && year > 1920 && year < *CUR_YEAR {
					debug!("year: {}, year_point: {:?}", year, file_name.len() - i - 1);
					self.year = Some(year);
					return Some(file_name.len() - i);
				} else {
					year = 0;
					digit_count = 0;
				}
				look_separator = false;
			} else if !look_separator && c.is_ascii_digit() {
				year += c.to_digit(10).unwrap() as u16 * 10u16.pow(digit_count);
				digit_count += 1;
			} else {
				year = 0;
				digit_count = 0;
				look_separator = true;
			}
		}
		None
	}

	fn split_by_separators<'a>(&self, file_name: &'a str) -> Vec<&'a str> {
		let mut parts = vec![];
		let mut left = 0;

		for (right, c) in file_name.char_indices() {
			let mut push_part = || {
				let part = &file_name[left..right];
				if !part.is_empty() {
					parts.push(part);
				}
				left = right + c.len_utf8();
			};

			if SEPARATORS.contains(&c) {
				push_part();
			} else if BRACKETS.contains(&c) {
				push_part();
				parts.push(&file_name[right..right + c.len_utf8()]);
			}
		}

		let part = &file_name[left..];
		if !part.is_empty() {
			parts.push(part);
		}

		parts
	}

	fn parse_file_name(&mut self) -> Result<()> {
		let file_name = Path::new(&self.path)
			.file_name()
			.ok_or_else(|| MediaOrderError::FilePathError(self.path.clone()))?
			.to_os_string()
			.into_string()?;

		// dbg!(&file_name);

		let mut parts = self.split_by_separators(&file_name);

		// remove extension
		let ext = parts.last().unwrap().to_lowercase();
		if VIDEO_EXTENSIONS.contains(&ext.as_str()) {
			self.ext = Some(ext);
			parts.pop();
		}

		// remove trash
		remove_trash(&mut parts);

		let mut name_end = parts.len();

		for (i, part) in parts.iter().enumerate().skip(1).rev() {
			if self.year.is_none() && part.len() == 4 {
				if let Ok(year) = part.parse::<u16>() {
					if year >= 1920 && year <= *CUR_YEAR {
						self.year = Some(year);
						name_end = i;
						continue;
					}
				}
			}

			let part = part.trim_end_matches(',').to_lowercase();

			if let Some(lang) = check_lang(&part, &LANG2) {
				match self.lang {
					Some(ref mut langs) => langs.push(lang),
					None => self.lang = Some(vec![lang]),
				}
				name_end = i;
				continue;
			}

			macro_rules! check_and_set {
				($field:ident, $set:expr) => {
					if self.$field.is_none() && $set.contains(&part.as_str()) {
						self.$field = Some(part);
						name_end = i;
						continue;
					}
				};
			}

			check_and_set!(venc, VENC);
			check_and_set!(aenc, AENC);
			check_and_set!(vres, VRES);
			check_and_set!(vqual, VQUAL);
		}

		let mut i = 0;
		while i < name_end {
			match parts[i] {
				"(" | "[" => {
					let close_bracket = if parts[i] == "(" { ")" } else { "]" };
					let part_in_brackets = parts[i + 1..name_end]
						.iter()
						.take_while(|&&part| part != close_bracket)
						.copied()
						.fold(String::new(), |mut acc, p| {
							if !acc.is_empty() {
								acc.push(' ');
							}
							acc.push_str(p);
							acc
						});
					if !part_in_brackets.is_empty() {
						if !self.name_original.is_empty() {
							self.name_original.push(' ');
						}
						self.name_original.push_str(parts[i]);
						self.name_original.push_str(&part_in_brackets);
						self.name_original.push_str(close_bracket);
					}
					i += part_in_brackets.split_whitespace().count() + 2;
				}
				_ => {
					if !self.name_original.is_empty() {
						self.name_original.push(' ');
					}
					self.name_original.push_str(parts[i]);
					i += 1;
				}
			}
		}

		// dbg!(&self.name_original);
		// dbg!(&parts);
		// dbg!(self);

		//TODO:
		// 1. Series detection (S\d\dE\d\d, seriya, serija, серия, ete., evristic (numbers in folder))
		// 2. Separate name part in brackets (also try to detect language in both parts)
		// 3. Removing commas
		Ok(())
	}

	pub fn get_full_ffmpeg_context(&self) -> String {
		let mut output = String::new();
		if let Some(ffmpeg_context) = &self.ffmpeg_context {
			for (k, v) in ffmpeg_context.metadata().iter() {
				output.push_str(&format!("{k}: {v}\n"));
			}

			if let Some(stream) = ffmpeg_context.streams().best(ffmpeg::media::Type::Video) {
				output.push_str(&format!("Best video stream index: {}\n", stream.index()));
			}

			if let Some(stream) = ffmpeg_context.streams().best(ffmpeg::media::Type::Audio) {
				output.push_str(&format!("Best audio stream index: {}\n", stream.index()));
			}

			if let Some(stream) = ffmpeg_context.streams().best(ffmpeg::media::Type::Subtitle) {
				output.push_str(&format!("Best subtitle stream index: {}\n", stream.index()));
			}

			output.push_str(&format!(
				"duration (seconds): {:.2}\n",
				ffmpeg_context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
			));

			for stream in ffmpeg_context.streams() {
				output.push_str(&format!("stream index {}:\n", stream.index()));
				output.push_str(&format!("\ttime_base: {}\n", stream.time_base()));
				output.push_str(&format!("\tstart_time: {}\n", stream.start_time()));
				output.push_str(&format!("\tduration (stream timebase): {}\n", stream.duration()));
				output.push_str(&format!(
					"\tduration (seconds): {:.2}\n",
					stream.duration() as f64 * f64::from(stream.time_base())
				));
				output.push_str(&format!("\tframes: {}\n", stream.frames()));
				output.push_str(&format!("\tdisposition: {:?}\n", stream.disposition()));
				output.push_str(&format!("\tdiscard: {:?}\n", stream.discard()));
				output.push_str(&format!("\trate: {}\n", stream.rate()));

				if let Ok(codec) = ffmpeg::codec::context::Context::from_parameters(stream.parameters()) {
					output.push_str(&format!("\tmedium: {:?}\n", codec.medium()));
					output.push_str(&format!("\tid: {:?}\n", codec.id()));

					match codec.medium() {
						ffmpeg::media::Type::Video => {
							if let Ok(video) = codec.decoder().video() {
								output.push_str(&format!("\tbit_rate: {}\n", video.bit_rate()));
								output.push_str(&format!("\tmax_rate: {}\n", video.max_bit_rate()));
								output.push_str(&format!("\tdelay: {}\n", video.delay()));
								output.push_str(&format!("\tvideo.width: {}\n", video.width()));
								output.push_str(&format!("\tvideo.height: {}\n", video.height()));
								output.push_str(&format!("\tvideo.format: {:?}\n", video.format()));
								output.push_str(&format!("\tvideo.has_b_frames: {}\n", video.has_b_frames()));
								output.push_str(&format!("\tvideo.aspect_ratio: {}\n", video.aspect_ratio()));
								output.push_str(&format!("\tvideo.color_space: {:?}\n", video.color_space()));
								output.push_str(&format!("\tvideo.color_range: {:?}\n", video.color_range()));
								output.push_str(&format!(
									"\tvideo.color_primaries: {:?}\n",
									video.color_primaries()
								));
								output.push_str(&format!(
									"\tvideo.color_transfer_characteristic: {:?}\n",
									video.color_transfer_characteristic()
								));
								output.push_str(&format!(
									"\tvideo.chroma_location: {:?}\n",
									video.chroma_location()
								));
								output.push_str(&format!("\tvideo.references: {}\n", video.references()));
								output.push_str(&format!(
									"\tvideo.intra_dc_precision: {}\n",
									video.intra_dc_precision()
								));
							}
						}
						ffmpeg::media::Type::Audio => {
							if let Ok(audio) = codec.decoder().audio() {
								output.push_str(&format!("\tbit_rate: {}\n", audio.bit_rate()));
								output.push_str(&format!("\tmax_rate: {}\n", audio.max_bit_rate()));
								output.push_str(&format!("\tdelay: {}\n", audio.delay()));
								output.push_str(&format!("\taudio.rate: {}\n", audio.rate()));
								output.push_str(&format!("\taudio.channels: {}\n", audio.channels()));
								output.push_str(&format!("\taudio.format: {:?}\n", audio.format()));
								output.push_str(&format!("\taudio.frames: {}\n", audio.frames()));
								output.push_str(&format!("\taudio.align: {}\n", audio.align()));
								output.push_str(&format!(
									"\taudio.channel_layout: {:?}\n",
									audio.channel_layout()
								));
							}
						}
						_ => {}
					}
				}
			}
		} else {
			output.push_str("ffmpeg_context is None\n");
		}
		output
	}
}

impl std::fmt::Debug for Video {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Video")
			.field("path", &self.path)
			.field("format", &self.format)
			.field("name_original", &self.name_original)
			.field("name_english", &self.name_english)
			.field("year", &self.year)
			.field("genre", &self.genre)
			.field("lang", &self.lang)
			.field("ext", &self.ext)
			.field("venc", &self.venc)
			.field("aenc", &self.aenc)
			.field("vres", &self.vres)
			.field("vqual", &self.vqual)
			.finish()
	}
}
