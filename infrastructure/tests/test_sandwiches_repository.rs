use ::entity::sandwich::Entity as Sandwich;
use entity::sandwich;
use infrastructure::sandwiches_repository::SandwichRepository;
use sea_orm::sea_query::TableCreateStatement;
use sea_orm::{entity::*, error::*, query::*, Database, DbBackend, DbConn, Schema};

#[tokio::test]
async fn test_get_all() -> Result<(), DbErr> {
    let db: DbConn = Database::connect("sqlite::memory:").await?;

    let schema = Schema::new(DbBackend::Sqlite);

    let stmt: TableCreateStatement = schema.create_table_from_entity(Sandwich);

    let _result = db
        .execute(db.get_database_backend().build(&stmt))
        .await
        .expect("Unable to create table");

    let sarnie = sandwich::ActiveModel {
        name: Set("Ham".to_owned()),
        count: Set(2),
        ..Default::default()
    };

    let _ = Sandwich::insert(sarnie)
        .exec(&db)
        .await
        .expect("Could not insert test data");

    let result = SandwichRepository::get_all(&db).await;
    assert_eq!(result[0].name, "Ham");

    Ok(())
}
