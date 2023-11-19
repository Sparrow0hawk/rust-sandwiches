use ::entity::sandwich;
use ::entity::sandwich::{Entity as Sandwich, Model};
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use sea_orm::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    sandwiches: Vec<Model>,
}

pub async fn index(data: web::Data<DatabaseConnection>) -> impl Responder {
    let sandwiches = SandwichRepository::get_all(&data).await;
    let title = "Rust Sandwiches";
    let template = IndexTemplate { title, sandwiches };

    HttpResponse::Ok().body(template.render().unwrap())
}

struct SandwichRepository;

impl SandwichRepository {
    pub async fn get_all(db: &DbConn) -> Vec<sandwich::Model> {
        Sandwich::find()
            .all(db)
            .await
            .expect("Unable to find sandwiches!")
    }
}
