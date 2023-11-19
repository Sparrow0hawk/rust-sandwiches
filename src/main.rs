use env_logger::Env;
use rust_sandwiches::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = rust_sandwiches::cli::parse_args().expect("Error parsing arguments");

    let address = format!("{}:{}", args.host, args.port);
    println!("App listening on http://{}", &address);

    let listener = TcpListener::bind(address)?;

    let server = run(listener).await?;

    server.await
}
