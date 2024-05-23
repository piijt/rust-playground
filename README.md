# CLI rust

## Docs

[how google rusts](https://google.github.io/comprehensive-rust/)
[rusts learning & docs entries](https://www.rust-lang.org/learn)

## setup

cargo new
cargo init

cargo run

- add `--release` for compiling for production

cargo build

- add `--release` for compiling for production

cargo clippy

rustc <PATH> -> invokes compiler directly

----------------------
## Formatting, code coverage and tests
rust has built in formatting just run `cargo fmt`
- this is standard across all packages config can be changed but this command is the end all solution

### test
`cargo test`

### Code coverage install
`cargo install cargo-llvm-cov`

### Remove potential conflicts with current report
`cargo llvm-cov clean`

### Run this to get code coverage report
`cargo llvm-cov --all-features --open`