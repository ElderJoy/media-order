use std::{fs, path::PathBuf};

use async_trait::async_trait;
use file_format::FileFormat;

use crate::{
	errors::{MediaOrderError, Result},
	video::Video,
};

#[async_trait]
pub trait FromPath: Sized {
	type Error;

	async fn from_path(path: PathBuf) -> Result<FSEntry>;
}

#[derive(Debug)]
pub enum FSEntry {
	Video(Video),
	File((PathBuf, FileFormat)),
	Folder(Vec<PathBuf>),
	Unknown(PathBuf),
}

#[async_trait]
impl FromPath for FSEntry {
	type Error = MediaOrderError;

	async fn from_path(path: PathBuf) -> Result<FSEntry> {
		match (path.is_dir(), FileFormat::from_file(&path)) {
			(true, _) => Ok(fs::read_dir(&path)
				.map(|entries| {
					FSEntry::Folder(
						entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).collect(),
					)
				})
				.unwrap_or_else(|_| FSEntry::Unknown(path))),
			(false, Ok(format)) => Ok(FSEntry::try_from((path, format))?),
			_ => Ok(FSEntry::Unknown(path)),
		}
	}
}

impl TryFrom<(PathBuf, FileFormat)> for FSEntry {
	type Error = MediaOrderError;

	fn try_from((path, format): (PathBuf, FileFormat)) -> Result<Self> {
		match format.kind() {
			file_format::Kind::Video => {
				let mut video = Video::new(path, format);
				video.read_ffmpeg_content()?;
				video.discover()?;
				Ok(Self::Video(video))
			}
			_ => Ok(Self::File((path, format))),
		}
	}
}

pub(crate) trait Explorer: std::fmt::Debug + Send + Sync {
	fn type_name(&self) -> &'static str;
}
