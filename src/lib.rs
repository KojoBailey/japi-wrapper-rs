use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::OnceLock;

static API: OnceLock<JoJoAPI> = OnceLock::new();

pub struct JoJoAPI {
    lib: Library,
}

impl JoJoAPI {
    fn load(dll_path: &str) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new(dll_path)?;
            Ok(JoJoAPI { lib })    
        }
    }
}

pub fn get() -> &'static JoJoAPI {
    API.get().expect("JoJoAPI has not been initialised; `ModInit` was not called.")
}

#[no_mangle]
pub extern "C" fn ModInit() {
    if API.get().is_some() {
        return;
    }

    let dll_path = "japi/dlls/JAPI.dll";
    let api = JoJoAPI::load(dll_path)
        .unwrap_or_else(|err| panic!("Failed to load JoJoAPI from {}: {}", dll_path, err));

    API.set(api).ok();
}
