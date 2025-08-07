use minhook::MinHook;

#[test]
fn test_remove_hook() {
    unsafe {
        MinHook::create_hook(test_fn as _, test_fn_hook as _).unwrap();

        // Remove the hook.
        MinHook::remove_hook(test_fn as _).unwrap();

        // Now enable all hooks
        MinHook::enable_all_hooks().unwrap();

        // Test that the hook is removed and never got enabled.

        assert_eq!(test_fn(), 0);
        assert_eq!(test_fn_hook(), 1);
    }

    fn test_fn() -> i32 {
        0
    }

    fn test_fn_hook() -> i32 {
        1
    }
}
