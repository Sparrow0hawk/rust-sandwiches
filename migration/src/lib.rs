pub use sea_orm_migration::prelude::*;

mod m20231119_200353_create_sandwiches_table;
mod m20231119_201322_add_initial_data_to_sandwiches_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231119_200353_create_sandwiches_table::Migration),
            Box::new(m20231119_201322_add_initial_data_to_sandwiches_table::Migration),
        ]
    }
}
