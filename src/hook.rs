use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::sync::atomic::AtomicPtr; 

#[link(name = "JAPI")]
unsafe extern "C" {
    fn JAPI_RegisterHook(hook_meta: JAPIHookMetaRaw) -> u64;
}

#[repr(C)]
struct JAPIHookMetaRaw {
    target: u64,
    detour: *const c_void,
    original: *mut *const c_void,
    name: *const c_char,
    is_game_function: bool,
}

pub fn register_hook<F>(
    target: u64,
    detour: F,
    original: &AtomicPtr<c_void>,
    name: &str,
    is_game_function: bool,
) -> Option<u64> {
    let c_name = CString::new(name).expect("Failed to create CString.").into_raw();

    let handle = unsafe {
        JAPI_RegisterHook(JAPIHookMetaRaw{
            target,
            detour: &detour as *const F as *const c_void,
            original: original.as_ptr() as *mut *const c_void,
            name: c_name,
            is_game_function,
        })
    };

    if handle == 0 {
        unsafe { let _ = CString::from_raw(c_name); } // Prevent memory leak on failure.
        return None;
    }
    Some(handle)
}
