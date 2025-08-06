use minhook::MinHook;
use std::ffi::c_void;

#[test]
fn test_hook() {
    unsafe {
        MinHook::create_hook(
            test_fn as FnType as *mut c_void,
            test_fn_hook as FnType as *mut c_void,
        )
        .unwrap();

        // Test that the hook is enabled.
        MinHook::enable_hook(test_fn as FnType as *mut c_void).unwrap();
        assert_eq!(test_fn(), 1);

        // Test that the hook is disabled.
        MinHook::disable_hook(test_fn as FnType as *mut c_void).unwrap();
        assert_eq!(test_fn(), 0);
    }

    type FnType = fn() -> i32;

    fn test_fn() -> i32 {
        0
    }

    fn test_fn_hook() -> i32 {
        1
    }
}
