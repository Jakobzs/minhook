use minhook::MinHook;

#[test]
fn test_hook() {
    // Minhook will automatically be initialized, so there is no need to ever call initialize().
    // We can explicitly uninitialize it once all detours and trampolines are no longer in use.
    unsafe {
        MinHook::enable_all_hooks().unwrap();
        MinHook::uninitialize().unwrap();

        // A later operation reinitializes MinHook instead of being stuck in an uninitialized state.
        MinHook::enable_all_hooks().unwrap();
        MinHook::uninitialize().unwrap();
    }
}
