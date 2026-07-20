# minhook

[![CI](https://github.com/Jakobzs/minhook/actions/workflows/test.yml/badge.svg)](https://github.com/Jakobzs/minhook/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/minhook)](https://crates.io/crates/minhook)
[![Documentation](https://img.shields.io/badge/docs-rustdoc-brightgreen)](https://jakobzs.github.io/minhook/minhook)
[![Codecov](https://codecov.io/github/Jakobzs/minhook/graph/badge.svg?token=PGVBSDVD83)](https://codecov.io/github/Jakobzs/minhook)
![MSRV](https://img.shields.io/badge/rust-1.85+-brightgreen.svg?logo=rust)

Rust bindings for [MinHook], a minimal x86/x64 API-hooking library for Windows.
The upstream C library is bundled and compiled automatically, so no separate
MinHook installation is required.

## Requirements

- Windows on x86 or x86-64
- Rust 1.85 or newer
- A C compiler supported by the [`cc`] crate

## Installation

Add the crate to your project:

```console
cargo add minhook
```

Or add it to `Cargo.toml` manually:

```toml
[dependencies]
minhook = "0.9.0"
```

## Example

The following example detours a function while retaining a pointer to its
original implementation:

```rust
use minhook::{MH_STATUS, MinHook};

fn main() -> Result<(), MH_STATUS> {
    // Create a hook for the return_0 function, detouring it to return_1
    let original = unsafe { MinHook::create_hook(return_0 as _, return_1 as _)? };

    // Enable the hook. We choose to call the "enable_all_hooks" function for the sake of the example
    unsafe { MinHook::enable_all_hooks()? };

    // Call the detoured return_0 function, it should return 1
    assert_eq!(return_0(), 1);

    // Transmute the original return_0 function address to a function pointer
    let original: fn() -> i32 = unsafe { std::mem::transmute(original) };

    // Call the original return_0 function
    assert_eq!(original(), 0);

    Ok(())
}

fn return_0() -> i32 {
    0
}

fn return_1() -> i32 {
    1
}
```

See the [API documentation] for creating API hooks, enabling or disabling
individual hooks, removing hooks, and applying queued operations.

## Safety

Most operations are `unsafe` because a hook changes executable code at runtime.
The target and detour must use compatible calling conventions and signatures,
and all pointers must remain valid for as long as the hook is active.

## Contributing

Contributions are welcome. Before opening a pull request, run:

```console
cargo fmt --all -- --check
cargo clippy --all-targets -- -D clippy::all
cargo test --all-targets
```

## License

This project is available under the [MIT License](LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project is licensed under the same terms.

[API documentation]: https://jakobzs.github.io/minhook/minhook
[MinHook]: https://github.com/TsudaKageyu/minhook
[`cc`]: https://crates.io/crates/cc
