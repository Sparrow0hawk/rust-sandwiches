use std::net::TcpListener;

use crate::routes::{index, not_found};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // load dotenv discard result
    dotenv().ok();

    let db_url = if dotenvy::var("DATABASE_URI").is_ok() {
        dotenvy::var("DATABASE_URI").unwrap()
    } else {
        String::from("sqlite::memory:")
    };

    let db_con = Database::connect(&db_url)
        .await
        .expect("Unable to connect to the database");
    Migrator::up(&db_con, None).await.expect("Migration failed");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_con.clone()))
            .route("/", web::get().to(index))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .default_service(web::get().to(not_found))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
