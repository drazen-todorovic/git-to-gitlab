debug:
	cargo clippy

release:
	cargo build --release --target x86_64-unknown-linux-musl
	upx --best --lzma target/x86_64-unknown-linux-musl/release/gtogl

update: release
	cp target/x86_64-unknown-linux-musl/release/gtogl ~/bin