use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::OnceLock;

#[repr(i32)] // Matches C default.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Fatal = 0,
    Error = 1,
    Warn  = 2,
    Info  = 3,
    Debug = 4,
    Trace = 5,
}

static API: OnceLock<JoJoAPI> = OnceLock::new();

pub struct JoJoAPI {
    lib: Library,
}

impl JoJoAPI {
    fn load() -> Result<Self, libloading::Error> {
        const DLL_PATH: &str = "japi/dlls/JAPI.dll";

        unsafe {
            let lib = Library::new(DLL_PATH)?;
            Ok(JoJoAPI { lib })    
        }
    }

    pub fn log(&self, level: LogLevel, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let c_str = CString::new(msg)?;
        unsafe {
            let func: Symbol<extern "C" fn(i32, *const c_char)> =
                self.lib.get(b"JAPI_LogMessage")?;
            func(level as i32, c_str.as_ptr());
        }
        Ok(())
    }
}

pub fn get() -> &'static JoJoAPI {
    API.get().expect("JoJoAPI has not been initialised; `ModInit` was not called.")
}

pub fn init() -> Result<(), libloading::Error> {
    if API.get().is_some() {
        return Ok(());
    }

    let api = JoJoAPI::load()?;
    API.set(api).ok();
}
