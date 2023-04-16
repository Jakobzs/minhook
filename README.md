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

This example shows how to create a hook for a function, and also call the original function.

```rust
use minhook::{MinHook, MH_STATUS};

fn main() -> Result<(), MH_STATUS> {
    // Create a hook for the test function
    let test_func_addr = unsafe { MinHook::create_hook(test as _, test_hook as _)? };

    // Enable the hook
    unsafe { MinHook::enable_all_hooks()? };

    // Call the detoured test function
    assert_eq!(test(), 1);

    // Transmute the original test function address to a function pointer
    let test_func = unsafe { std::mem::transmute::<_, fn() -> i32>(test_func_addr) };

    // Call the original test function
    assert_eq!(test_func(), 0);

    Ok(())
}

fn test() -> i32 {
    0
}

fn test_hook() -> i32 {
    1
}
```

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
