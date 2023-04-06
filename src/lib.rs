use ffi::{
    MH_ApplyQueued, MH_CreateHook, MH_CreateHookApi, MH_CreateHookApiEx, MH_DisableHook,
    MH_EnableHook, MH_Initialize, MH_QueueDisableHook, MH_QueueEnableHook, MH_RemoveHook,
    MH_Uninitialize,
};
use once_cell::sync::OnceCell;
use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};
use tracing::debug;

mod ffi;

const MH_ALL_HOOKS: *const i32 = std::ptr::null();

static MINHOOK_INIT: OnceCell<()> = OnceCell::new();
static MINHOOK_UNINIT: OnceCell<()> = OnceCell::new();

pub struct MinHook {}

impl MinHook {
    fn initialize() {
        MINHOOK_INIT.get_or_init(|| unsafe {
            let status = MH_Initialize();
            debug!("MH_Initialize: {:?}", status);

            status.ok().expect("Couldn't initialize MinHook");
        });
    }

    /// # Safety
    pub fn uninitialize() {
        // Make sure we are initialized before we uninitialize
        Self::initialize();

        MINHOOK_UNINIT.get_or_init(|| unsafe {
            let status = MH_Uninitialize();
            debug!("MH_Uninitialize: {:?}", status);

            status.ok().expect("Couldn't uninitialize MinHook");
        });
    }

    /// # Safety
    ///
    /// This can be made safe if target and detour can verified as valid function pointers and will not be freed
    pub unsafe fn create_hook(
        target: *mut c_void,
        detour: *mut c_void,
    ) -> Result<*mut c_void, MH_STATUS> {
        Self::initialize();

        let mut pp_original: *mut c_void = null_mut();
        let status = unsafe { MH_CreateHook(target, detour, &mut pp_original) };
        debug!("MH_CreateHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(pp_original),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn create_hook_api<T: AsRef<str>>(
        module_name: T,
        proc_name: T,
        detour: *mut c_void,
    ) -> Result<*mut c_void, MH_STATUS> {
        Self::initialize();

        let module_name = CString::new(module_name.as_ref()).unwrap();
        let proc_name = CString::new(proc_name.as_ref()).unwrap();
        let mut pp_original: *mut c_void = null_mut();
        let status = unsafe {
            MH_CreateHookApi(
                module_name.as_ptr() as *const _,
                proc_name.as_ptr() as *const _,
                detour,
                &mut pp_original,
            )
        };
        debug!("MH_CreateHookApi: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(pp_original),
            _ => Err(status),
        }
    }

    /// # Safety
    ///
    /// TOOO: Revise if this is correct
    pub unsafe fn create_hook_api_ex<T: AsRef<str>>(
        module_name: T,
        proc_name: T,
        detour: *mut c_void,
    ) -> Result<(*mut c_void, *mut *mut c_void), MH_STATUS> {
        Self::initialize();

        let module_name = CString::new(module_name.as_ref()).unwrap();
        let proc_name = CString::new(proc_name.as_ref()).unwrap();
        let mut pp_original: *mut c_void = null_mut();
        let pp_target: *mut *mut c_void = null_mut();
        let status = unsafe {
            MH_CreateHookApiEx(
                module_name.as_ptr() as *const _,
                proc_name.as_ptr() as *const _,
                detour,
                &mut pp_original,
                pp_target,
            )
        };
        debug!("MH_CreateHookApiEx: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok((pp_original, pp_target)),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn enable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_EnableHook(target) };
        debug!("MH_EnableHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn enable_all_hooks() -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_EnableHook(MH_ALL_HOOKS as *mut _) };
        debug!("MH_EnableHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn disable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_DisableHook(target) };
        debug!("MH_DisableHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn disable_all_hooks() -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_DisableHook(MH_ALL_HOOKS as *mut _) };
        debug!("MH_DisableHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn remove_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_RemoveHook(target) };
        debug!("MH_RemoveHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn queue_enable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_QueueEnableHook(target) };
        debug!("MH_QueueEnableHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn queue_disable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_QueueDisableHook(target) };
        debug!("MH_QueueDisableHook: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn apply_queued() -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_ApplyQueued() };
        debug!("MH_ApplyQueued: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }
}

#[allow(non_camel_case_types)]
#[must_use]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MH_STATUS {
    /// Unknown error. Should not be returned.
    MH_UNKNOWN = -1,
    /// Successful.
    MH_OK = 0,
    /// MinHook is already initialized.
    MH_ERROR_ALREADY_INITIALIZED,
    /// MinHook is not initialized yet, or already uninitialized.
    MH_ERROR_NOT_INITIALIZED,
    /// The hook for the specified target function is already created.
    MH_ERROR_ALREADY_CREATED,
    /// The hook for the specified target function is not created yet.
    MH_ERROR_NOT_CREATED,
    /// The hook for the specified target function is already enabled.
    MH_ERROR_ENABLED,
    /// The hook for the specified target function is not enabled yet, or
    /// already disabled.
    MH_ERROR_DISABLED,
    /// The specified pointer is invalid. It points the address of non-allocated
    /// and/or non-executable region.
    MH_ERROR_NOT_EXECUTABLE,
    /// The specified target function cannot be hooked.
    MH_ERROR_UNSUPPORTED_FUNCTION,
    /// Failed to allocate memory.
    MH_ERROR_MEMORY_ALLOC,
    /// Failed to change the memory protection.
    MH_ERROR_MEMORY_PROTECT,
    /// The specified module is not loaded.
    MH_ERROR_MODULE_NOT_FOUND,
    /// The specified function is not found.
    MH_ERROR_FUNCTION_NOT_FOUND,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::c_void, mem};

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

    #[test]
    fn test_hooks_queue() {
        unsafe {
            MinHook::create_hook(
                test_fn1 as fn() -> i32 as *mut c_void,
                test_fn1_hook as fn() -> i32 as *mut c_void,
            )
            .unwrap();
            MinHook::create_hook(
                test_fn2 as fn(i32) -> i32 as *mut c_void,
                test_fn2_hook as fn(i32) -> i32 as *mut c_void,
            )
            .unwrap();

            // Queue to enable the hooks, then apply them.
            MinHook::queue_enable_hook(test_fn1 as fn() -> i32 as *mut c_void).unwrap();
            MinHook::queue_enable_hook(test_fn2 as fn(i32) -> i32 as *mut c_void).unwrap();
            MinHook::apply_queued().unwrap();

            // Test that the hooks are enabled.
            assert_eq!(test_fn1(), 1);
            assert_eq!(test_fn2(1), 2);

            // Queue to disable the hooks, then apply them.
            MinHook::queue_disable_hook(test_fn1 as fn() -> i32 as *mut c_void).unwrap();
            MinHook::queue_disable_hook(test_fn2 as fn(i32) -> i32 as *mut c_void).unwrap();
            MinHook::apply_queued().unwrap();

            // Test that the hooks are disabled.
            assert_eq!(test_fn1(), 0);
            assert_eq!(test_fn2(1), 1);
        }

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

    #[test]
    fn test_hook_trampoline() {
        unsafe {
            // Create a hook for `test_fn_trampoline_orig`
            let trampoline = MinHook::create_hook(
                test_fn_trampoline_orig as fn(i32) -> i32 as *mut c_void,
                test_fn_trampoline_hook as fn(i32) -> i32 as *mut c_void,
            )
            .unwrap();

            // Store the trampoline function.
            TRAMPOLINE.get_or_init(|| mem::transmute(trampoline));

            // Enable the hook.
            MinHook::enable_hook(test_fn_trampoline_orig as fn(i32) -> i32 as *mut c_void).unwrap();

            assert_eq!(test_fn_trampoline_orig(69), 42)
        }

        type FnType = fn(i32) -> i32;
        static TRAMPOLINE: OnceCell<FnType> = OnceCell::new();

        fn test_fn_trampoline_orig(x: i32) -> i32 {
            x
        }

        fn test_fn_trampoline_hook(_x: i32) -> i32 {
            // Set a value that we want to return for the test.
            let val = 42;

            // Call the trampoline function with the new value.
            TRAMPOLINE.get().unwrap()(val)
        }
    }
}
