build: src/main.rs
	RUSTFLAGS="" RUST_TARGET_PATH=$(pwd) cargo +nightly xbuild --target x86_64-none-efi --package uefi-app

install-deps:
	rustup toolchain install nightly
	rustup default nightly
	rustup component add --toolchain nightly rust-src
	cargo install --force cargo-xbuild
