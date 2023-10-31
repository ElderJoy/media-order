use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Titles::Table)
					.if_not_exists()
					.col(ColumnDef::new(Titles::Title).string().not_null())
					.col(ColumnDef::new(Titles::Id).integer().not_null())
					.primary_key(Index::create().col(Titles::Title).col(Titles::Id))
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.if_not_exists()
					.name("idx_title")
					.table(Titles::Table)
					.col(Titles::Title)
					.to_owned(),
			)
			.await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_index(Index::drop().name("idx_title").table(Titles::Table).to_owned())
			.await?;

		manager.drop_table(Table::drop().table(Titles::Table).to_owned()).await?;

		Ok(())
	}
}

#[derive(DeriveIden)]
enum Titles {
	Table,
	Title,
	Id,
}
