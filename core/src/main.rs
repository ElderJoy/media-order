mod errors;
mod types;
pub mod video;

#[macro_use] extern crate lazy_static;

use std::{env, path::PathBuf};

use log::debug;
use simple_logger::SimpleLogger;
use tokio::task::JoinSet;
use types::{FSEntry, FromPath};
use video::Video;

use crate::errors::{MediaOrderError, Result};

#[tokio::main]
async fn main() -> Result<()> {
	SimpleLogger::new().init().unwrap();
	Video::init();

	dotenvy::dotenv().ok();
	let video_library_path = env::var("VIDEO_LIBRARY_PATH").expect("VIDEO_LIBRARY_PATH is not set");

	let path = PathBuf::from(video_library_path);
	let mut set = JoinSet::new();
	set.spawn(FSEntry::from_path(path));

	while let Some(entry) = set.join_next().await {
		debug!("{:#?}", entry);
		if let Ok(FSEntry::Folder(entries)) = entry.map_err(MediaOrderError::JoinError)? {
			for entry in entries {
				set.spawn(FSEntry::from_path(entry));
			}
		}
	}

	Ok(())
}
