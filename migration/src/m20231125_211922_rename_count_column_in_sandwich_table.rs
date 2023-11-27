use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Sandwich::Table)
                    .rename_column(Alias::new("count"), Alias::new("amount"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Sandwich::Table)
                    .rename_column(Alias::new("amount"), Alias::new("count"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Sandwich {
    Table,
}
