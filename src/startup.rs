use std::net::TcpListener;

use crate::routes::{index, not_found};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    dotenv().expect("Unable to load .env file");
    let db_url =
        dotenvy::var("DATABASE_URI").expect("Unable to load DATABASE_URI environment variable");
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
