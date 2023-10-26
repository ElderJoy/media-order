use crate::local::title;

#[test]
fn parse_gzip_title() {
	let titles = title::parse_gzip_file("src/tests/local/title.akas.tsv.gz");

	assert_eq!(titles.len(), 772);
	assert_eq!(titles[0].title_id, "tt13522842");
	assert_eq!(titles[0].ordering, 1);
	assert_eq!(titles[0].title, "एपिसोड #1.3980");
	assert_eq!(titles[0].region, Some("IN".to_owned()));
	assert_eq!(titles[0].language, Some("hi".to_owned()));
	assert_eq!(titles[0].types, Some("\\N".to_string()));
	assert_eq!(titles[0].attributes, Some("\\N".to_string()));
	assert_eq!(titles[0].is_original_title, false);
}
