[![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://github.com/vshymanskyy/StandWithUkraine/blob/main/docs/README.md)

# denote-rs

Use [denote](https://github.com/protesilaos/denote) name shcheme outside of
Emacs. You can watch [this](https://youtu.be/mLzFJcLpDFI) video to find out
about Denote name scheme and originally plugin to Emacs.

## Installation

### Cargo

You need [rust toolchain](https://www.rust-lang.org/tools/install) for install
denote-rs from Crates.io.

```
cargo install zeroten-denote-cli
denote --version
```

### Pre-built binaries

See the [release section](https://github.com/5121f/denote-rs/releases) for
manual installation of a binary.

## Building

denote-rs written in Rust so you need
[Rust toolchain](https://www.rust-lang.org/tools/install) for build it.

To build denote-rs:

```
git clone https://github.com/5121f/denote-rs.git
cd denote-rs
cargo build --release
target/release/denote --version
```

## TODO

- Changing front matter

## About me

I'm a amateur programmer and English is not my native language so I will be glad
to get any corrections and feedback from you.

## License

Most of the files in this project are dual-licensed under **Mozilla Public
License 2.0 (MPL 2.0) and the GNU General Public License version 2 or
later (GPL-2.0-or-later)**, at the user's option.

The full texts of the licenses can be found in the [`LICENSES`](./LICENSES) directory.
