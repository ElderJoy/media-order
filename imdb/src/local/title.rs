use std::io::{BufRead, BufReader};

use log::debug;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_until, take_while_m_n},
	IResult,
};

#[derive(Debug)]
pub struct Title {
	pub title_id: String,
	pub ordering: u8,
	pub name: String,
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

	let mut titles = Vec::new();
	for line in BufReader::new(gzip).lines().flatten() {
		debug!("{}", line);
		match title(&line) {
			Ok((_, title)) => {
				debug!("{:#?}", &title);
				titles.push(title);
			}
			Err(e) => {
				debug!("Error parsing imdb title file: {}", e);
			}
		}
	}

	titles
}

fn title(input: &str) -> IResult<&str, Title> {
	let (input, title_id) = title_id(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, ordering) = ordering(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, name) = name(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, region) = region(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, language) = language(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, types) = types(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, attributes) = attributes(input)?;
	let (input, _) = tag("\t")(input)?;
	let (input, is_original_title) = is_original_title(input)?;

	Ok((input, Title {
		title_id: title_id.to_string(),
		ordering,
		name: name.to_string(),
		region: region.map(|s| s.to_string()),
		language: language.map(|s| s.to_string()),
		types: types.map(|s| s.to_string()),
		attributes: attributes.map(|s| s.to_string()),
		is_original_title,
	}))
}

fn title_id(input: &str) -> IResult<&str, &str> {
	let (input, _) = tag("tt")(input)?;
	let (input, id) = take_while_m_n(7, 8, |c: char| c.is_ascii_digit())(input)?;
	Ok((input, id))
}

fn ordering(input: &str) -> IResult<&str, u8> {
	let (input, ordering) = take_while_m_n(1, 2, |c: char| c.is_ascii_digit())(input)?;
	Ok((input, ordering.parse().unwrap()))
}

fn name(input: &str) -> IResult<&str, &str> {
	let (input, name) = take_until("\t")(input)?;
	Ok((input, name))
}

fn none(input: &str) -> IResult<&str, &str> {
	tag("\\N")(input)
}

fn region(input: &str) -> IResult<&str, Option<&str>> {
	let (input, region) = alt((none, take_while_m_n(1, 4, |c: char| c.is_ascii_uppercase())))(input)?;
	Ok((input, if region == "\\N" { None } else { Some(region) }))
}

fn language(input: &str) -> IResult<&str, Option<&str>> {
	let (input, language) = alt((none, take_while_m_n(1, 3, |c: char| c.is_ascii_lowercase())))(input)?;
	Ok((input, if language == "\\N" { None } else { Some(language) }))
}

fn types(input: &str) -> IResult<&str, Option<&str>> {
	let (input, types) = alt((none, take_while_m_n(1, 100, |c: char| c.is_ascii_alphanumeric())))(input)?;
	Ok((input, if types == "\\N" { None } else { Some(types) }))
}

fn attributes(input: &str) -> IResult<&str, Option<&str>> {
	let (input, attributes) = alt((none, take_until("\t")))(input)?;
	Ok((input, if attributes == "\\N" { None } else { Some(attributes) }))
}

fn is_original_title(input: &str) -> IResult<&str, bool> {
	let (input, is_original_title) = alt((tag("0"), tag("1")))(input)?;
	Ok((input, is_original_title == "1"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_title_id() {
		assert_eq!(title_id("tt1234567"), Ok(("", "1234567")));
		assert_eq!(title_id("tt12345678"), Ok(("", "12345678")));
		assert_eq!(title_id("tt123456789"), Ok(("9", "12345678")));
		assert_eq!(
			title_id("tt123456"),
			Err(nom::Err::Error(nom::error::Error {
				input: "123456",
				code: nom::error::ErrorKind::TakeWhileMN
			}))
		);
	}
}
