rust-version:
	rustc --version 		# rustc compiler
	cargo --version 		# cargo package manager
	rustfmt --version 		# rust formatter
	rustup --version 		# rust toolchain manager
	clippy-driver --version	# rust linter

format:
	cargo fmt

lint: format 
	cargo clippy

test:
	cargo test

prep: test lint

run:
	cargo run -r

build:
	cargo update
	cargo build 
	
release:
	cargo update
	cargo build -r
	strip target/release/matrix

clean:
	cargo clean
	rm -rf target

install: release
	cp target/release/matrix ~/.local/bin

uninstall:
	rm -f ~/.local/bin/matrix

.PHONY: format lint test run build release clean install uninstall prep