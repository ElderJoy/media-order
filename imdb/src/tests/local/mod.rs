use simple_logger::SimpleLogger;

use crate::local::title;

#[test]
fn parse_gzip_title() {
	SimpleLogger::new().init().unwrap();
	let file_path =
		std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/local/title.akas.tsv.gz");
	let titles = title::parse_gzip_file(&file_path);

	assert_eq!(titles.len(), 772);
	assert_eq!(titles[0].title_id, "13522842");
	assert_eq!(titles[0].ordering, 1);
	assert_eq!(titles[0].name, "एपिसोड #1.3980");
	assert_eq!(titles[0].region, Some("IN".to_owned()));
	assert_eq!(titles[0].language, Some("hi".to_owned()));
	assert_eq!(titles[0].types, None);
	assert_eq!(titles[0].attributes, None);
	assert_eq!(titles[0].is_original_title, false);
}
