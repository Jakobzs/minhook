# minhook

[![CI](https://github.com/Jakobzs/minhook/actions/workflows/test.yml/badge.svg)](https://github.com/Jakobzs/minhook/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/minhook)](https://crates.io/crates/minhook)
[![rustdoc](https://img.shields.io/badge/docs-rustdoc-brightgreen)](https://jakobzs.github.io/minhook/minhook)
[![codecov](https://codecov.io/github/Jakobzs/minhook/graph/badge.svg?token=PGVBSDVD83)](https://codecov.io/github/Jakobzs/minhook)
![MSRV](https://img.shields.io/badge/rust-1.85+-brightgreen.svg?&logo=rust)

A Rust wrapper for the [MinHook](https://github.com/TsudaKageyu/minhook) library.

## Usage

Add `minhook` by using the following command:

```bash
cargo add minhook
```

Or if you prefer to add it manually, you can do so by editing your `Cargo.toml` file:

```toml
[dependencies]
minhook = "0.9.0"
```

## Example

This example shows how to create a hook for a function, and also call the original function.

```rust
use minhook::{MinHook, MH_STATUS};

fn main() -> Result<(), MH_STATUS> {
    // Keep calls indirect so optimized builds cannot bypass the patched entry point.
    let return_0 = std::hint::black_box(return_0 as fn() -> i32);
    let return_1 = std::hint::black_box(return_1 as fn() -> i32);

    // Create a hook for the return_0 function, detouring it to return_1.
    let return_0_address = unsafe { MinHook::create_hook(return_0 as _, return_1 as _)? };

    // Enable the hook
    unsafe { MinHook::enable_all_hooks()? };

    // Call the detoured return_0 function, it should return 1
    assert_eq!(return_0(), 1);

    // Transmute the original return_0 function address to a function pointer
    let return_0_original = unsafe { std::mem::transmute::<_, fn() -> i32>(return_0_address) };

    // Call the original return_0 function
    assert_eq!(return_0_original(), 0);

    Ok(())
}

#[inline(never)]
fn return_0() -> i32 {
    0
}

#[inline(never)]
fn return_1() -> i32 {
    1
}
```

Calls to functions defined in the same Rust crate can be optimized without going through the
patched entry point. Keep such functions out of line and call them through an opaque function
pointer, as the example does, when testing hooks in optimized builds.

When hooking a Win32 API, the detour and trampoline must use the API's exact signature and calling
convention. In most cases this means `unsafe extern "system" fn(...)`. This is especially important
on 32-bit Windows, where the system ABI differs from Rust's native ABI.

`MinHook::uninitialize` is unsafe because it can invalidate every trampoline allocated by the
native library. Call it only after all detours and trampoline calls have stopped. The wrapper will
not tear down a MinHook instance that it detected was initialized by another component.

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
