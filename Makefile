target/x86_64-none-efi/debug/uefi-app.efi: src/main.rs Cargo.toml
	cargo +nightly xbuild --target x86_64-unknown-uefi --package uefi-app

install-deps:
	rustup toolchain install nightly
	rustup default nightly
	rustup component add --toolchain nightly rust-src
	cargo install --force cargo-xbuild

run-qemu:
	rm -rf efi/
	mkdir -p efi/EFI/BOOT/
	cp target/x86_64-unknown-uefi/debug/uefi-app.efi efi/EFI/BOOT/BOOTX64.EFI
	qemu-kvm \
		-nodefaults \
		-vga std \
		-machine q35,accel=kvm:tcg \
		-m 128M \
		-drive if=pflash,format=raw,readonly,file=/usr/share/OVMF/OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=OVMF_VARS.fd \
		-drive format=raw,file=fat:rw:efi/ \
		-serial stdio \
		-monitor vc:1024x768
