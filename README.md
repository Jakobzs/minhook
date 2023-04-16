# minhook

[![Rust](https://github.com/Jakobzs/minhook/actions/workflows/rust.yml/badge.svg)](https://github.com/Jakobzs/minhook/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/minhook)](https://crates.io/crates/minhook)
[![rustdoc](https://img.shields.io/badge/docs-rustdoc-brightgreen)](https://docs.rs/minhook)

A Rust wrapper for the [MinHook](https://github.com/TsudaKageyu/minhook) library.

Unlike other detouring crates, this crate does **not** require nightly. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
minhook = "0.1.0"
```

## Example

Please refer to the tests for examples on how to use this crate for the time being.

```rust
fn main() -> Result<(), MH_STATUS> {
// WIP
}
```

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
