FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release --bin rust_sandwiches && mv ./target/release/rust_sandwiches ./rust_sandwiches

# Runtime image
FROM debian:bookworm-slim as main

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app
# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/rust_sandwiches /app/rust_sandwiches

# Run the app
CMD ./rust_sandwiches --port 8080 --host 0.0.0.0
