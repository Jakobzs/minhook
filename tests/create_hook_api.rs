use minhook::MinHook;

#[test]
fn test_create_hook_api() {
    // Hook get process id function
    unsafe {
        // Create a hook for the `GetCurrentProcessId` function using the extended API
        let original = MinHook::create_hook_api(
            "kernel32.dll",
            "GetCurrentProcessId",
            get_current_process_id_hook as _,
        )
        .unwrap();

        // Grab the current process id
        let original_pid = std::process::id();

        // Enable all hooks (this is necessary since we do not have a handle to the GetCurrentProcessId function)
        MinHook::enable_all_hooks().unwrap();

        // Call the Rust std library function to get the current process id
        // It should return the value we set in the hook `get_current_process_id_hook`
        assert_eq!(std::process::id(), 42);

        // Transmute the original function address to a function pointer to make it callable
        let original_fn: fn() -> u32 = std::mem::transmute(original);

        // Call the original function using the original function pointer
        assert_eq!(original_fn(), original_pid);

        // Disable the hook
        MinHook::disable_all_hooks().unwrap();

        // Ensure the original function is restored
        assert_eq!(std::process::id(), original_pid);
    }

    fn get_current_process_id_hook() -> u32 {
        42
    }
}
