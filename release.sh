#!/usr/bin/env sh

version="0.1.1"
name="denote"
linux_target="x86_64-unknown-linux-gnu"
windows_target="x86_64-pc-windows-gnu"

cargo build --release --target $linux_target
cargo build --release --target $windows_target
tar cf $name-$version-$linux_target.tar.gz --directory=target/$linux_target/release $name
zip -j $name-$version-$windows_target.zip target/$windows_target/release/$name.exe
