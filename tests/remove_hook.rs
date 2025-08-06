use minhook::MinHook;
use std::ffi::c_void;

#[test]
fn test_remove_hook() {
    unsafe {
        MinHook::create_hook(
            test_fn as FnType as *mut c_void,
            test_fn_hook as FnType as *mut c_void,
        )
        .unwrap();

        // Remove the hook.
        MinHook::remove_hook(test_fn as FnType as *mut c_void).unwrap();

        // Now enable all hooks
        MinHook::enable_all_hooks().unwrap();

        // Test that the hook is removed and never got enabled.

        assert_eq!(test_fn(), 0);
        assert_eq!(test_fn_hook(), 1);
    }

    type FnType = fn() -> i32;

    fn test_fn() -> i32 {
        0
    }

    fn test_fn_hook() -> i32 {
        1
    }
}
