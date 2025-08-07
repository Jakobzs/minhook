use minhook::MinHook;
use std::ffi::c_void;

#[test]
fn test_hooks_queue() {
    unsafe {
        MinHook::create_hook(
            test_fn1 as FnType1 as *mut c_void,
            test_fn1_hook as FnType1 as *mut c_void,
        )
        .unwrap();
        MinHook::create_hook(
            test_fn2 as FnType2 as *mut c_void,
            test_fn2_hook as FnType2 as *mut c_void,
        )
        .unwrap();

        // Queue to enable the hooks, then apply them.
        MinHook::queue_enable_hook(test_fn1 as FnType1 as *mut c_void).unwrap();
        MinHook::queue_enable_hook(test_fn2 as FnType2 as *mut c_void).unwrap();
        MinHook::apply_queued().unwrap();

        // Test that the hooks are enabled.
        assert_eq!(test_fn1(), 1);
        assert_eq!(test_fn2(1), 2);

        // Queue to disable the hooks, then apply them.
        MinHook::queue_disable_hook(test_fn1 as FnType1 as *mut c_void).unwrap();
        MinHook::queue_disable_hook(test_fn2 as FnType2 as *mut c_void).unwrap();
        MinHook::apply_queued().unwrap();

        // Test that the hooks are disabled.
        assert_eq!(test_fn1(), 0);
        assert_eq!(test_fn2(1), 1);
    }

    type FnType1 = fn() -> i32;
    type FnType2 = fn(i32) -> i32;

    fn test_fn1() -> i32 {
        0
    }

    fn test_fn1_hook() -> i32 {
        1
    }

    fn test_fn2(x: i32) -> i32 {
        x
    }

    fn test_fn2_hook(x: i32) -> i32 {
        x + 1
    }
}
