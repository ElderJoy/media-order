use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImdbError {
	#[error(transparent)]
	SeaOrmMigration(#[from] sea_orm_migration::DbErr),
}

pub type Result<T> = std::result::Result<T, ImdbError>;
