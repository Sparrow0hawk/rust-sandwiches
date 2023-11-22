use ::entity::sandwich;
use ::entity::sandwich::Entity as Sandwich;
use sea_orm::*;

pub struct SandwichRepository;

impl SandwichRepository {
    pub async fn get_all(db: &DbConn) -> Vec<sandwich::Model> {
        Sandwich::find()
            .all(db)
            .await
            .expect("Unable to find sandwiches!")
    }
}
