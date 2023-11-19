use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Sandwich::Table)
            .columns([Sandwich::Name, Sandwich::Count])
            .values_panic(["Marmite and Cheese".into(), "10".into()])
            .values_panic(["Peanut Butter and Jam".into(), "5".into()])
            .values_panic(["Melty Cheese".into(), "100".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let query = Query::delete().from_table(Sandwich::Table).to_owned();
        manager.exec_stmt(query).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Sandwich {
    Table,
    #[allow(dead_code)]
    Id,
    Name,
    Count,
}
