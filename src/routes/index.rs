use ::entity::sandwich::Model;
use ::infrastructure::sandwiches_repository::SandwichRepository;
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
    let sandwiches = SandwichRepository::new(&data).get_all().await;
    let title = "Rust Sandwiches";
    let template = IndexTemplate { title, sandwiches };

    HttpResponse::Ok().body(template.render().unwrap())
}
