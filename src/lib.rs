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
//!     // Keep calls indirect so optimized builds cannot bypass the patched entry point.
//!     let return_0 = std::hint::black_box(return_0 as fn() -> i32);
//!     let return_1 = std::hint::black_box(return_1 as fn() -> i32);
//!
//!     // Create a hook for the return_0 function, detouring it to return_1.
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
//! #[inline(never)]
//! fn return_0() -> i32 {
//!     0
//! }
//!
//! #[inline(never)]
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
    fmt,
    ptr::null_mut,
    sync::{Mutex, MutexGuard},
};
use tracing::debug;

mod ffi;

const MH_ALL_HOOKS: *const i32 = std::ptr::null();

#[derive(Debug)]
struct MinHookState {
    initialized: bool,
    owned: bool,
}

static MINHOOK_STATE: Mutex<MinHookState> = Mutex::new(MinHookState {
    initialized: false,
    owned: false,
});

/// A struct to access the MinHook API.
pub struct MinHook {}

impl MinHook {
    fn state() -> MutexGuard<'static, MinHookState> {
        MINHOOK_STATE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    // Initialize MinHook and retain the state lock for the native operation that follows.
    fn initialize() -> Result<MutexGuard<'static, MinHookState>, MH_STATUS> {
        let mut state = Self::state();
        if state.initialized {
            return Ok(state);
        }

        let status = unsafe { MH_Initialize() };
        debug!("MH_Initialize: {:?}", status);

        match status {
            MH_STATUS::MH_OK => {
                state.initialized = true;
                state.owned = true;
                Ok(state)
            }
            MH_STATUS::MH_ERROR_ALREADY_INITIALIZED => {
                // Another component owns the process-global MinHook instance. We can use it,
                // but must not tear it down from `uninitialize`.
                state.initialized = true;
                state.owned = false;
                Ok(state)
            }
            _ => Err(status),
        }
    }

    /// Detaches this wrapper from MinHook and uninitializes the native library when this
    /// wrapper initialized it.
    ///
    /// A later API call may initialize MinHook again. If MinHook was already initialized by
    /// another component, this method only resets the wrapper's state and leaves the shared
    /// native instance running.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no thread is executing a detour or trampoline and that no
    /// trampoline pointer returned by this crate will be called after this method succeeds.
    /// All other users of the process-global MinHook instance must be synchronized with this
    /// operation.
    pub unsafe fn uninitialize() -> Result<(), MH_STATUS> {
        let mut state = Self::state();
        if !state.initialized {
            return Err(MH_STATUS::MH_ERROR_NOT_INITIALIZED);
        }

        if !state.owned {
            state.initialized = false;
            return Ok(());
        }

        let status = unsafe { MH_Uninitialize() };
        debug!("MH_Uninitialize: {:?}", status);

        if status == MH_STATUS::MH_OK {
            state.initialized = false;
            state.owned = false;
        }

        status.ok()
    }

    /// Creates a hook for the target function and detours it to the detour function. This function returns the original function pointer.
    ///
    /// # Safety
    ///
    /// `target` and `detour` must be valid executable function addresses with identical
    /// signatures and calling conventions. Neither function may unwind across an incompatible
    /// ABI boundary. The returned trampoline may only be called while the hook exists and
    /// MinHook remains initialized.
    pub unsafe fn create_hook(
        target: *mut c_void,
        detour: *mut c_void,
    ) -> Result<*mut c_void, MH_STATUS> {
        let _state = Self::initialize()?;

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
    ///
    /// `detour` must have exactly the same signature and calling convention as the named API.
    /// Win32 API detours should normally use `extern "system"`. The detour must not unwind across
    /// the foreign ABI boundary. The returned trampoline may only be called while the hook exists
    /// and MinHook remains initialized.
    pub unsafe fn create_hook_api<T: AsRef<str>>(
        module_name: T,
        proc_name: T,
        detour: *mut c_void,
    ) -> Result<*mut c_void, MH_STATUS> {
        let _state = Self::initialize()?;

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
    /// `detour` must have exactly the same signature and calling convention as the named API.
    /// Win32 API detours should normally use `extern "system"`. The detour must not unwind across
    /// the foreign ABI boundary. The returned pointers remain valid only while the hook exists,
    /// its module remains loaded, and MinHook remains initialized.
    pub unsafe fn create_hook_api_ex<T: AsRef<str>>(
        module_name: T,
        proc_name: T,
        detour: *mut c_void,
    ) -> Result<(*mut c_void, *mut c_void), MH_STATUS> {
        let _state = Self::initialize()?;

        let mut module_name = module_name.as_ref().encode_utf16().collect::<Vec<_>>();
        module_name.push(0);

        let proc_name = CString::new(proc_name.as_ref()).unwrap();
        let mut pp_original: *mut c_void = null_mut();
        let mut pp_target: *mut c_void = null_mut();
        let status = unsafe {
            MH_CreateHookApiEx(
                module_name.as_ptr() as *const _,
                proc_name.as_ptr() as *const _,
                detour,
                &mut pp_original,
                &mut pp_target,
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
    ///
    /// `target` must identify a hook created by MinHook. The caller must ensure that activating
    /// the detour is sound for every thread that can call the target function.
    pub unsafe fn enable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        let _state = Self::initialize()?;

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
    ///
    /// The caller must ensure that activating every registered detour is sound for every thread
    /// that can call the affected target functions.
    pub unsafe fn enable_all_hooks() -> Result<(), MH_STATUS> {
        unsafe { Self::enable_hook(MH_ALL_HOOKS as *mut _) }
    }

    /// Disables a hook for the target function.
    ///
    /// # Safety
    ///
    /// `target` must identify a hook created by MinHook. The caller must synchronize any code
    /// that depends on whether the detour is active.
    pub unsafe fn disable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        let _state = Self::initialize()?;

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
    ///
    /// The caller must synchronize any code that depends on whether registered detours are
    /// active.
    pub unsafe fn disable_all_hooks() -> Result<(), MH_STATUS> {
        unsafe { Self::disable_hook(MH_ALL_HOOKS as *mut _) }
    }

    /// Removes a hook for the target function.
    ///
    /// # Safety
    ///
    /// `target` must identify a hook created by MinHook. No thread may subsequently call the
    /// trampoline returned when that hook was created.
    pub unsafe fn remove_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        let _state = Self::initialize()?;

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
    ///
    /// `target` must identify a hook created by MinHook. The caller must ensure that activating
    /// the detour will be sound when the queued operation is applied.
    pub unsafe fn queue_enable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        let _state = Self::initialize()?;

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
    ///
    /// `target` must identify a hook created by MinHook. The caller must synchronize any code
    /// that depends on whether the detour is active when the queued operation is applied.
    pub unsafe fn queue_disable_hook(target: *mut c_void) -> Result<(), MH_STATUS> {
        let _state = Self::initialize()?;

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
    ///
    /// The caller must satisfy the safety requirements of every queued enable or disable
    /// operation and synchronize code that can execute the affected functions.
    pub unsafe fn apply_queued() -> Result<(), MH_STATUS> {
        let _state = Self::initialize()?;

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

impl fmt::Display for MH_STATUS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            MH_STATUS::MH_UNKNOWN => "Unknown error. Should not be returned.",
            MH_STATUS::MH_OK => "Successful.",
            MH_STATUS::MH_ERROR_ALREADY_INITIALIZED => "MinHook is already initialized.",
            MH_STATUS::MH_ERROR_NOT_INITIALIZED => {
                "MinHook is not initialized yet, or already uninitialized."
            }
            MH_STATUS::MH_ERROR_ALREADY_CREATED => {
                "The hook for the specified target function is already created."
            }
            MH_STATUS::MH_ERROR_NOT_CREATED => {
                "The hook for the specified target function is not created yet."
            }
            MH_STATUS::MH_ERROR_ENABLED => {
                "The hook for the specified target function is already enabled."
            }
            MH_STATUS::MH_ERROR_DISABLED => {
                "The hook for the specified target function is not enabled yet, or already disabled."
            }
            MH_STATUS::MH_ERROR_NOT_EXECUTABLE => {
                "The specified pointer is invalid. It points the address of non-allocated and/or non-executable region."
            }
            MH_STATUS::MH_ERROR_UNSUPPORTED_FUNCTION => {
                "The specified target function cannot be hooked."
            }
            MH_STATUS::MH_ERROR_MEMORY_ALLOC => "Failed to allocate memory.",
            MH_STATUS::MH_ERROR_MEMORY_PROTECT => "Failed to change the memory protection.",
            MH_STATUS::MH_ERROR_MODULE_NOT_FOUND => "The specified module is not loaded.",
            MH_STATUS::MH_ERROR_FUNCTION_NOT_FOUND => "The specified function is not found.",
        };

        write!(f, "{message}")
    }
}

impl std::error::Error for MH_STATUS {}
