pub mod entities;
pub mod migration;
pub mod title;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

use crate::{errors::Result, local::migration::MigratorTrait};

pub async fn create_database(db_file_name: &str) -> Result<DatabaseConnection> {
	let db_url = format!("sqlite://{}.sqlite?mode=rwc", db_file_name);
	let connect_options = ConnectOptions::new(db_url).to_owned();

	let db = Database::connect(connect_options).await?;
	migration::Migrator::up(&db, None).await?;
	Ok(db)
}

pub async fn open_database(db_file_name: &str) -> Result<DatabaseConnection> {
	let db_url = format!("sqlite://{}.sqlite?mode=rwc", db_file_name);
	let connect_options = ConnectOptions::new(db_url).to_owned();

	let db = Database::connect(connect_options).await?;
	Ok(db)
}
