use minhook::MinHook;

#[test]
fn test_hook() {
    unsafe {
        MinHook::create_hook(test_fn as _, test_fn_hook as _).unwrap();

        // Test that the hook is enabled.
        MinHook::enable_hook(test_fn as _).unwrap();
        assert_eq!(test_fn(), 1);

        // Test that the hook is disabled.
        MinHook::disable_hook(test_fn as _).unwrap();
        assert_eq!(test_fn(), 0);
    }

    fn test_fn() -> i32 {
        0
    }

    fn test_fn_hook() -> i32 {
        1
    }
}
