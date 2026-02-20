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

pub fn get_module_base_address() -> *const u8 {
    unsafe { JAPI_GetModuleBaseAddress() as *const u8 }
}

pub fn offset_to_module_address(relative_address: usize) -> *const u8 {
    unsafe { get_module_base_address().add(relative_address) }
}
