use std::{ffi::OsString, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaOrderError {
	#[error("Can't read video metadata")]
	VideoMetadata,
	#[error(transparent)]
	JoinError(#[from] tokio::task::JoinError),
	#[error("File path error {0}")]
	FilePathError(PathBuf),
	#[error("Error converting filename from OsString {:?} to String", 0)]
	OsStringError(OsString),
}

impl From<OsString> for MediaOrderError {
	fn from(err: OsString) -> Self {
		MediaOrderError::OsStringError(err)
	}
}

pub type Result<T> = std::result::Result<T, MediaOrderError>;
