build-release:
    cargo build --release

install: build-release
    cp target/release/denote-rs ~/.local/bin
