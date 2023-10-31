pub mod migration;
pub mod title;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

use crate::{errors::Result, local::migration::MigratorTrait};

pub async fn create_database() -> Result<DatabaseConnection> {
	let connect_options = ConnectOptions::new("sqlite://local_imdb.sqlite?mode=rwc")
		.set_schema_search_path("my_schema")
		.to_owned();

	let db = Database::connect(connect_options).await?;
	migration::Migrator::up(&db, None).await?;
	Ok(db)
}
