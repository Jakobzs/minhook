//! [MinHook](https://github.com/TsudaKageyu/minhook) is a Windows API hooking library that allows you to intercept calls to functions in programs.
//!
//! This crate is a wrapper around the MinHook library. Most of the API is unsafe because it is not possible to guarantee safety of the hooks.
//!
//! # Example
//!
//! This example shows how to create a hook for a function, and also call the original function.
//!
//! ```rust
//! use minhook::{MinHook, MH_STATUS};
//!
//! fn main() -> Result<(), MH_STATUS> {
//!     // Create a hook for the return_0 function, detouring it to return_1
//!     let return_0_address = unsafe { MinHook::create_hook(return_0 as _, return_1 as _)? };
//!
//!     // Enable the hook
//!     unsafe { MinHook::enable_all_hooks()? };
//!
//!     // Call the detoured return_0 function, it should return 1
//!     assert_eq!(return_0(), 1);
//!
//!     // Transmute the original return_0 function address to a function pointer
//!     let return_0_original = unsafe { std::mem::transmute::<_, fn() -> i32>(return_0_address) };
//!
//!     // Call the original return_0 function
//!     assert_eq!(return_0_original(), 0);
//!
//!     Ok(())
//! }
//!
//! fn return_0() -> i32 {
//!     0
//! }
//!
//! fn return_1() -> i32 {
//!     1
//! }
//! ```

use ffi::{
    MH_ApplyQueued, MH_CreateHook, MH_CreateHookApi, MH_CreateHookApiEx, MH_DisableHook,
    MH_EnableHook, MH_Initialize, MH_QueueDisableHook, MH_QueueEnableHook, MH_RemoveHook,
    MH_Uninitialize,
};
use std::{
    ffi::{CString, c_void},
    ptr::null_mut,
    sync::Once,
};
use tracing::debug;

mod ffi;

const MH_ALL_HOOKS: *const i32 = std::ptr::null();

static MINHOOK_INIT: Once = Once::new();
static MINHOOK_UNINIT: Once = Once::new();

/// A struct to access the MinHook API.
pub struct MinHook {}

impl MinHook {
    // Initialize MinHook
    fn initialize() {
        MINHOOK_INIT.call_once(|| {
            let status = unsafe { MH_Initialize() };
            debug!("MH_Initialize: {:?}", status);

            match status.ok() {
                Ok(_) => (), // Initialization successful, do nothing
                Err(MH_STATUS::MH_ERROR_ALREADY_INITIALIZED) => (), // Ignore if already initialized
                Err(e) => panic!("Could not initialize MinHook, error: {e:?}"),
            }
        });
    }

    /// Uninitializes MinHook. This function is only possible to call once. If you want to reinitialize MinHook, you need to restart the program.
    ///
    /// # Safety
    pub fn uninitialize() {
        // Make sure we are initialized before we uninitialize
        Self::initialize();

        MINHOOK_UNINIT.call_once(|| {
            let status = unsafe { MH_Uninitialize() };
            debug!("MH_Uninitialize: {:?}", status);

            status.ok().expect("Could not uninitialize MinHook");
        });
    }

    /// Creates a hook for the target function and detours it to the detour function. This function returns the original function pointer.
    ///
    /// # Safety
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

    /// Creates a hook for the targeted API function and detours it to the detour function. This function returns the original function pointer.
    ///
    /// # Safety
    pub unsafe fn create_hook_api<T: AsRef<str>>(
        module_name: T,
        proc_name: T,
        detour: *mut c_void,
    ) -> Result<*mut c_void, MH_STATUS> {
        Self::initialize();

        let mut module_name = module_name.as_ref().encode_utf16().collect::<Vec<_>>();
        module_name.push(0);

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

    /// Extended function for creating a hook for the targeted API function and detours it to the detour function. This function returns the original function pointer as well as a pointer to the target function.
    /// # Safety
    ///
    /// TOOO: Revise if this is correct
    pub unsafe fn create_hook_api_ex<T: AsRef<str>>(
        module_name: T,
        proc_name: T,
        detour: *mut c_void,
    ) -> Result<(*mut c_void, *mut *mut c_void), MH_STATUS> {
        Self::initialize();

        let mut module_name = module_name.as_ref().encode_utf16().collect::<Vec<_>>();
        module_name.push(0);

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

    /// Enables a hook for the target function.
    ///
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

    /// Enables all hooks.
    ///
    /// # Safety
    pub unsafe fn enable_all_hooks() -> Result<(), MH_STATUS> {
        unsafe { Self::enable_hook(MH_ALL_HOOKS as *mut _) }
    }

    /// Disables a hook for the target function.
    ///
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

    /// Disables all hooks.
    ///
    /// # Safety
    pub unsafe fn disable_all_hooks() -> Result<(), MH_STATUS> {
        unsafe { Self::disable_hook(MH_ALL_HOOKS as *mut _) }
    }

    /// Removes a hook for the target function.
    ///
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

    /// Queues a hook for enabling.
    ///
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

    /// Queues a hook for disabling.
    ///
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

    /// Applies all queued hooks.
    ///
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

/// MinHook status codes.
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

impl MH_STATUS {
    pub fn ok(self) -> Result<(), MH_STATUS> {
        if self == MH_STATUS::MH_OK {
            Ok(())
        } else {
            Err(self)
        }
    }
}
