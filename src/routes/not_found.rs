use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "not_found.html")]
struct PageNotFoundTemplate{}

pub async fn not_found() -> impl Responder {
    let template = PageNotFoundTemplate{};

    HttpResponse::NotFound().body(template.render().unwrap())
}
