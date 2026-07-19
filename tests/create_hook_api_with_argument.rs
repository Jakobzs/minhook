use minhook::MinHook;
use std::sync::atomic::{AtomicBool, Ordering};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn Sleep(milliseconds: u32);
}

static DETOUR_CALLED: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn sleep_hook(_milliseconds: u32) {
    DETOUR_CALLED.store(true, Ordering::SeqCst);
}

#[test]
fn test_create_hook_api_with_system_abi_argument() {
    unsafe {
        let (original, target) =
            MinHook::create_hook_api_ex("kernel32.dll", "Sleep", sleep_hook as _).unwrap();
        let original: unsafe extern "system" fn(u32) = std::mem::transmute(original);

        MinHook::enable_hook(target).unwrap();
        Sleep(0);
        assert!(DETOUR_CALLED.swap(false, Ordering::SeqCst));

        // Calling the trampoline must preserve the same system ABI and bypass the detour.
        original(0);
        assert!(!DETOUR_CALLED.load(Ordering::SeqCst));

        MinHook::disable_hook(target).unwrap();
        MinHook::remove_hook(target).unwrap();
    }
}
