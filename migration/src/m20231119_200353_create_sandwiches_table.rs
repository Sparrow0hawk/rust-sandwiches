use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sandwich::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sandwich::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sandwich::Name).string().not_null())
                    .col(ColumnDef::new(Sandwich::Count).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sandwich::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sandwich {
    Table,
    Id,
    Name,
    Count,
}
