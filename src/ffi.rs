use crate::MH_STATUS;
use std::ffi::c_void;

extern "system" {
    /// Initializes the MinHook library. You must call this function in the
    /// beginning of your program.
    pub fn MH_Initialize() -> MH_STATUS;

    /// Uninitialize the MinHook library. You must call this function EXACTLY
    /// ONCE at the end of your program.
    pub fn MH_Uninitialize() -> MH_STATUS;

    /// Creates a hook for the specified target function, in disabled state.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function, which will be overridden by the detour function.
    /// * `pDetour` \[in\] - A pointer to the detour function, which will override the target function.
    /// * `ppOriginal` \[out\] - A pointer to the trampoline function, which will be used to call the original target function. This parameter can be NULL.
    pub fn MH_CreateHook(
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
    pub fn MH_CreateHookApi(
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
    pub fn MH_CreateHookApiEx(
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
    pub fn MH_RemoveHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Enables an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    pub fn MH_EnableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Disables an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    pub fn MH_DisableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Queues to enable an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    pub fn MH_QueueEnableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Queues to disable an already created hook.
    ///
    /// # Arguments
    ///
    /// * `pTarget` \[in\] - A pointer to the target function.
    pub fn MH_QueueDisableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Applies all queued changes in one go.
    pub fn MH_ApplyQueued() -> MH_STATUS;
}
