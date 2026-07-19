use minhook::{MH_STATUS, MinHook};

unsafe extern "system" {
    fn MH_Initialize() -> MH_STATUS;
    fn MH_Uninitialize() -> MH_STATUS;
}

#[test]
fn wrapper_does_not_uninitialize_an_external_owner() {
    unsafe {
        assert_eq!(MH_Initialize(), MH_STATUS::MH_OK);

        // The wrapper observes an already initialized native library and borrows it.
        MinHook::enable_all_hooks().unwrap();
        MinHook::uninitialize().unwrap();

        // If the wrapper had torn down borrowed state, this would report NOT_INITIALIZED.
        assert_eq!(MH_Uninitialize(), MH_STATUS::MH_OK);
    }
}
