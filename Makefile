# centralise all the commands I need to remember

clippy:
	cargo clippy

format:
	cargo fmt

test:
	cargo test --workspace

check:
	cargo check

verify: check test format clippy