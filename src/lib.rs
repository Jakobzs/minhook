#![allow(dead_code, non_snake_case, non_camel_case_types)]

use once_cell::sync::OnceCell;
use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};
use tracing::debug;

const MH_ALL_HOOKS: *const i32 = std::ptr::null();

static MINHOOK_INIT: OnceCell<()> = OnceCell::new();
static MINHOOK_UNINIT: OnceCell<()> = OnceCell::new();

pub struct MinHook {}

impl MinHook {
    unsafe fn initialize() {
        MINHOOK_INIT.get_or_init(|| unsafe {
            let status = MH_Initialize();
            debug!("MH_Initialize: {:?}", status);

            status.ok().expect("Couldn't initialize MinHook");
        });
    }

    /// # Safety
    pub unsafe fn uninitialize() {
        // Make sure we are initialized
        Self::initialize();

        MINHOOK_UNINIT.get_or_init(|| unsafe {
            let status = MH_Uninitialize();
            debug!("MH_Uninitialize: {:?}", status);

            status.ok().expect("Couldn't uninitialize MinHook");
        });
    }

    /// # Safety
    pub unsafe fn create_hook(
        target: *mut c_void,
        detour: *mut c_void,
    ) -> Result<*mut c_void, MH_STATUS> {
        Self::initialize();

        let mut original: *mut c_void = null_mut();
        let status = unsafe { MH_CreateHook(target, detour, &mut original) };
        debug!("create_hook: {:?} {:?} {:?}", target, detour, original);
        match status {
            MH_STATUS::MH_OK => Ok(original),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn enable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_EnableHook(target) };
        debug!("enable_hook: {:?} {:?}", target, status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn enable_all_hooks() -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_EnableHook(MH_ALL_HOOKS as *mut _) };
        debug!("enable_all_hooks: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn disable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_DisableHook(target) };
        debug!("disable_hook: {:?} {:?}", target, status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn disable_all_hooks() -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_DisableHook(MH_ALL_HOOKS as *mut _) };
        debug!("disable_all_hooks: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn remove_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_RemoveHook(target) };
        debug!("remove_hook: {:?} {:?}", target, status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn queue_enable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_QueueEnableHook(target) };
        debug!("queue_enable_hook: {:?} {:?}", target, status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn queue_disable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_QueueDisableHook(target) };
        debug!("queue_disable_hook: {:?} {:?}", target, status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn apply_queued() -> Result<(), MH_STATUS> {
        Self::initialize();

        let status = unsafe { MH_ApplyQueued() };
        debug!("apply_queued: {:?}", status);
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn create_hook_api(
        module_name: &str,
        proc_name: &str,
        detour: *mut c_void,
    ) -> Result<(), MH_STATUS> {
        Self::initialize();

        let module_name = CString::new(module_name).unwrap();
        let proc_name = CString::new(proc_name).unwrap();
        let mut original: *mut c_void = null_mut();
        let status = unsafe {
            MH_CreateHookApi(
                module_name.as_ptr() as *const _,
                proc_name.as_ptr() as *const _,
                detour,
                &mut original,
            )
        };
        debug!(
            "create_hook_api: {:?} {:?} {:?} {:?}",
            module_name, proc_name, detour, original
        );
        match status {
            MH_STATUS::MH_OK => Ok(()),
            _ => Err(status),
        }
    }

    /// # Safety
    pub unsafe fn create_hook_api_ex(
        module_name: &str,
        proc_name: &str,
        detour: *mut c_void,
        pp_target: *mut *mut c_void,
    ) -> Result<(), MH_STATUS> {
        Self::initialize();

        let module_name = CString::new(module_name).unwrap();
        let proc_name = CString::new(proc_name).unwrap();
        let mut original: *mut c_void = null_mut();
        let status = unsafe {
            MH_CreateHookApiEx(
                module_name.as_ptr() as *const _,
                proc_name.as_ptr() as *const _,
                detour,
                &mut original,
                pp_target,
            )
        };
        debug!(
            "create_hook_api_ex: {:?} {:?} {:?} {:?} {:?}",
            module_name, proc_name, detour, original, pp_target
        );
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

extern "system" {
    /// Initializes the MinHook library. You must call this function in the
    /// beginning of your program.
    fn MH_Initialize() -> MH_STATUS;

    /// Uninitialize the MinHook library. You must call this function EXACTLY
    /// ONCE at the end of your program.
    fn MH_Uninitialize() -> MH_STATUS;

    /// Creates a hook for the specified target function, in disabled state.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function, which will be overridden by the detour function.
    /// * `pDetour` \[in\] - A pointer to the detour function, which will override the target function.
    /// * `ppOriginal` \[out\] - A pointer to the trampoline function, which will be used to call the original target function. This parameter can be NULL.
    fn MH_CreateHook(
        pTarget: *mut c_void,
        pDetour: *mut c_void,
        ppOriginal: *mut *mut c_void,
    ) -> MH_STATUS;

    /// Creates a hook for the specified API function, in disabled state.
    ///
    /// # Arguments
    ///
    /// * `pszModule` \[in\] - A pointer to the loaded module name which contains the target function.
    /// * `pszProcName` \[in\] - A pointer to the target function name, which will be overridden by the detour function.
    /// * `pDetour` \[in\] - A pointer to the detour function, which will override the target function.
    /// * `ppOriginal` \[out\] - A pointer to the trampoline function, which will be used to call the original target function. This parameter can be NULL.
    fn MH_CreateHookApi(
        pszModule: *const u8,
        pszProcName: *const u8,
        pDetour: *mut c_void,
        ppOriginal: *mut *mut c_void,
    ) -> MH_STATUS;

    /// Creates a hook for the specified API function, in disabled state.
    ///
    /// # Arguments
    ///
    /// * `pszModule` \[in\] - A pointer to the loaded module name which contains the target function.
    /// * `pszProcName` \[in\] - A pointer to the target function name, which will be overridden by the detour function.
    /// * `pDetour` \[in\] - A pointer to the detour function, which will override the target function.
    /// * `ppOriginal` \[out\] - A pointer to the trampoline function, which will be used to call the original target function. This parameter can be NULL.
    /// * `ppTarget` \[out\] - A pointer to the target function, which will be overridden by the detour function. This parameter can be NULL.
    fn MH_CreateHookApiEx(
        pszModule: *const u8,
        pszProcName: *const u8,
        pDetour: *mut c_void,
        ppOriginal: *mut *mut c_void,
        ppTarget: *mut *mut c_void,
    ) -> MH_STATUS;

    /// Removes an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    fn MH_RemoveHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Enables an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    fn MH_EnableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Disables an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    fn MH_DisableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Queues to enable an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    fn MH_QueueEnableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Queues to disable an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    fn MH_QueueDisableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Applies all queued changes in one go.
    fn MH_ApplyQueued() -> MH_STATUS;
}

impl MH_STATUS {
    pub fn ok(self) -> Result<(), MH_STATUS> {
        if self == MH_STATUS::MH_OK {
            Ok(())
        } else {
            Err(self)
        }
    }
}

/// Structure that holds original address, hook function address, and trampoline
/// address for a given hook.
pub struct MhHook {
    addr: *mut c_void,
    hook_impl: *mut c_void,
    trampoline: *mut c_void,
}

static INIT_CELL: OnceCell<()> = OnceCell::new();

impl MhHook {
    /// Create a new hook.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address of the function to hook.
    /// * `hook_impl` - Address of the function to call instead of `addr`.
    ///
    /// # Returns
    ///
    /// A `MhHook` struct that holds the original address, hook function address,
    /// and trampoline address for the given hook.
    ///
    /// # Safety
    ///
    /// `addr` must be a valid address to a function.
    /// `hook_impl` must be a valid address to a function.
    pub unsafe fn new(addr: *mut c_void, hook_impl: *mut c_void) -> Result<Self, MH_STATUS> {
        INIT_CELL.get_or_init(|| {
            let status = unsafe { MH_Initialize() };
            debug!("MH_Initialize: {:?}", status);

            status.ok().expect("Couldn't initialize hooks");
        });

        let mut trampoline = null_mut();
        let status = MH_CreateHook(addr, hook_impl, &mut trampoline);
        debug!("MH_CreateHook: {:?}", status);

        status.ok()?;

        Ok(Self {
            addr,
            hook_impl,
            trampoline,
        })
    }

    pub fn trampoline(&self) -> *mut c_void {
        self.trampoline
    }

    unsafe fn queue_enable(&self) {
        let status = MH_QueueEnableHook(self.hook_impl);
        debug!("MH_QueueEnableHook: {:?}", status);
    }

    unsafe fn queue_disable(&self) {
        let status = MH_QueueDisableHook(self.hook_impl);
        debug!("MH_QueueDisableHook: {:?}", status);
    }
}

/// Wrapper for a queue of hooks to be applied via Minhook.
pub struct MhHooks(Vec<MhHook>);
unsafe impl Send for MhHooks {}
unsafe impl Sync for MhHooks {}

impl MhHooks {
    pub fn new<T: IntoIterator<Item = MhHook>>(hooks: T) -> Result<Self, MH_STATUS> {
        Ok(MhHooks(hooks.into_iter().collect::<Vec<_>>()))
    }

    pub fn apply(&self) {
        unsafe { MhHooks::apply_hooks(&self.0) };
    }

    pub fn unapply(&self) {
        unsafe { MhHooks::unapply_hooks(&self.0) };
        let status = unsafe { MH_Uninitialize() };
        debug!("MH_Uninitialize: {:?}", status);
    }

    unsafe fn apply_hooks(hooks: &[MhHook]) {
        for hook in hooks {
            let status = MH_QueueEnableHook(hook.addr);
            debug!("MH_QueueEnable: {:?}", status);
        }
        let status = MH_ApplyQueued();
        debug!("MH_ApplyQueued: {:?}", status);
    }

    unsafe fn unapply_hooks(hooks: &[MhHook]) {
        for hook in hooks {
            let status = MH_QueueDisableHook(hook.addr);
            debug!("MH_QueueDisable: {:?}", status);
        }
        let status = MH_ApplyQueued();
        debug!("MH_ApplyQueued: {:?}", status);
    }
}

impl Drop for MhHooks {
    fn drop(&mut self) {
        // self.unapply();
    }
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

            MinHook::queue_enable_hook(test_fn1 as fn() -> i32 as *mut c_void).unwrap();
            MinHook::queue_enable_hook(test_fn2 as fn(i32) -> i32 as *mut c_void).unwrap();
            MinHook::apply_queued().unwrap();

            // Test that the hooks are applied.
            assert_eq!(test_fn1(), 1);
            assert_eq!(test_fn2(1), 2);

            MinHook::queue_disable_hook(test_fn1 as fn() -> i32 as *mut c_void).unwrap();
            MinHook::queue_disable_hook(test_fn2 as fn(i32) -> i32 as *mut c_void).unwrap();
            MinHook::apply_queued().unwrap();

            // Test that the hooks are unapplied.
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

            // Call the trampoline function.
            let trampoline = TRAMPOLINE.get().unwrap();
            trampoline(val)
        }
    }
}
