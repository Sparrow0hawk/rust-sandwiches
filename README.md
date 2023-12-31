# Actix Web Sandwich Shop

Testing out template rendering and ORM tools with Actix-web.

### Main dependencies

- [Actix web](https://actix.rs)
- [lexopt](https://docs.rs/lexopt/latest/lexopt/) for CLI interface
- [Askama](https://docs.rs/askama/latest/askama/) for Jinja-like templating
- GitHub actions that depend on https://github.com/dtolnay/rust-toolchain
- [SeaORM](https://www.sea-ql.org/SeaORM/) for ORM, database operations, migrations
- Sqlite, for database

## Usage

To use this repository you will need:
- Rust

### Local

To run locally:

1. Run cargo

    ```bash
    cargo run
    ```
    
    Visit http://127.0.0.1:8080 to view the app.
    
    Optionally run on a different port
    
    ```bash
    cargo run -- -p 5000
    ```
    
    View other CLI options with:
    
    ```bash
    cargo run -- --help
    Usage: actix_starter [-h|--host=X.X.X.X] [-p|--port=XXXX]
    ```

2. To run the tests:

   ```bash
   cargo test
   ```

### Docker

If you have Docker installed you can build the app with the following commands:

1. Build the image
   ```bash
   docker build . -t rusty_sandwiches
   ```

2. Run the image
   ```bash
   docker run --rm -p 8080:8080 rusty_sandwiches:latest
   ```
