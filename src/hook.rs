use std::os::raw::{c_char, c_void};
use std::ffi::CString;

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

#[macro_export]
macro_rules! register_hook {
    (
        $addr:expr,
        $hook:expr,
        $original:expr,
        $name:expr,
        $is_game_function:expr
    ) => {{
        unsafe {
            $crate::register_hook_impl(
                $addr,
                $hook as *const std::os::raw::c_void,
                &raw mut $original as *mut *const std::os::raw::c_void,
                $name,
                $is_game_function
            )
        }
    }};
}

pub fn register_hook_impl(
    target: u64,
    detour: *const c_void,
    original: *mut *const c_void,
    name: &str,
    is_game_function: bool,
) -> Option<u64> {
    let c_name = CString::new(name).expect("Failed to create CString.").into_raw();

    let handle = unsafe {
        JAPI_RegisterHook(JAPIHookMetaRaw{
            target,
            detour,
            original,
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
