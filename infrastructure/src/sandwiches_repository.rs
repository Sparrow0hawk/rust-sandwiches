use ::entity::sandwich;
use ::entity::sandwich::Entity as Sandwich;
use sea_orm::*;

pub struct SandwichRepository<'a> {
    db: &'a DbConn,
}

impl<'a> SandwichRepository<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        SandwichRepository { db }
    }

    pub async fn get_all(self) -> Vec<sandwich::Model> {
        Sandwich::find()
            .all(self.db)
            .await
            .expect("Unable to find sandwiches!")
    }
}
