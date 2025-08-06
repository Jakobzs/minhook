use minhook::MinHook;
use once_cell::sync::OnceCell;
use std::{ffi::c_void, mem};

#[test]
fn test_hook_trampoline_enable_and_disable_hook() {
    unsafe {
        // Create a hook for `test_fn_trampoline_orig`
        let trampoline = MinHook::create_hook(
            test_fn_trampoline_orig as FnType as *mut c_void,
            test_fn_trampoline_hook as FnType as *mut c_void,
        )
        .unwrap();

        // Store the trampoline function.
        TRAMPOLINE.get_or_init(|| mem::transmute(trampoline));

        // Enable the hook.
        MinHook::enable_hook(test_fn_trampoline_orig as FnType as *mut c_void).unwrap();

        assert_eq!(test_fn_trampoline_orig(69), 42);

        // Disable the hook.
        MinHook::disable_hook(test_fn_trampoline_orig as FnType as *mut c_void).unwrap();

        assert_eq!(test_fn_trampoline_orig(69), 69);
    }

    type FnType = fn(i32) -> i32;
    static TRAMPOLINE: OnceCell<FnType> = OnceCell::new();

    fn test_fn_trampoline_orig(x: i32) -> i32 {
        x
    }

    fn test_fn_trampoline_hook(_x: i32) -> i32 {
        // Set a value that we want to return for the test.
        let val = 42;

        // Call the trampoline function with the new value.
        TRAMPOLINE.get().unwrap()(val)
    }
}
