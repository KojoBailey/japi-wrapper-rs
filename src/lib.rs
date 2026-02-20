mod mod_meta;
mod log;
mod hook;

pub use mod_meta::*;
pub use log::*;
pub use hook::*;

#[link(name = "JAPI")]
unsafe extern "C" {
    fn JAPI_GetModuleBaseAddress() -> *const u64;
}

pub fn get_module_base_address() -> *const u64 {
    unsafe { JAPI_GetModuleBaseAddress() }
}
