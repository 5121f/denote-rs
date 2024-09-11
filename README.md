[![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://github.com/vshymanskyy/StandWithUkraine/blob/main/docs/README.md)

# denote-rs

Use [denote](https://github.com/protesilaos/denote) name shcheme outside of Emacs.

## Building

denote-rs written in Rust so you need [Rust compiler and Cargo](https://www.rust-lang.org) for build it.

To build denote-rs:

```
$ git clone https://github.com/5121f/denote-rs.git
$ cd denote-rs
$ cargo build --release
$ target/release/denote-rs --version
0.1.0
```

## TODO

- Changing front matter
- Rename file based on it's front matter
