use minhook::MinHook;

#[test]
fn test_hook() {
    // Minhook will automatically be initialized, so there is no need to ever call initialize().
    // However, we can call uninitialize() when we are done with MinHook.
    // It is not unsafe to call uninitialize(), even multiple times, but it will only uninitialize MinHook once.
    MinHook::uninitialize();
}
