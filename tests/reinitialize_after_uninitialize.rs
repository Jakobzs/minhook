use minhook::{MH_STATUS, MinHook};

unsafe extern "system" {
    fn MH_Initialize() -> MH_STATUS;
    fn MH_Uninitialize() -> MH_STATUS;
}

#[test]
fn wrapper_reinitializes_native_minhook_after_uninitialize() {
    unsafe {
        // A public wrapper operation performs the first native initialization.
        MinHook::enable_all_hooks().unwrap();
        MinHook::uninitialize().unwrap();

        // Native MinHook confirms that the wrapper completely uninitialized it.
        assert_eq!(MH_Uninitialize(), MH_STATUS::MH_ERROR_NOT_INITIALIZED);

        // A subsequent wrapper operation must initialize the native library again.
        MinHook::enable_all_hooks().unwrap();

        // Native MinHook confirms that it is initialized for the second time.
        assert_eq!(MH_Initialize(), MH_STATUS::MH_ERROR_ALREADY_INITIALIZED);

        MinHook::uninitialize().unwrap();
    }
}
