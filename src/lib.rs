use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::OnceLock;

#[repr(C)]
pub struct JAPIModMetaRaw {
    name: *const c_char,
    author: *const c_char,
    guid: *const c_char,
    version: *const c_char,
    description: *const c_char,
}

#[macro_export]
macro_rules! register_mod {
    (
        title: $title:expr,
        author: $author:expr,
        guid: $guid:expr,
        version: $version:expr,
        desc: $desc:expr,
    ) => {
        static MOD_META_STORAGE: ::std::sync::OnceLock<$crate::ModMetaStorage> = ::std::sync::OnceLock::new();

        #[unsafe(no_mangle)]
        pub extern "C" fn GetModMeta() -> $crate::JAPIModMetaRaw {
            let storage = MOD_META_STORAGE.get_or_init(|| $crate::ModMetaStorage::new(
                    $title,
                    $author,
                    $guid,
                    $version,
                    $desc,
            ));
            storage.as_raw();
        }
    };
}

pub struct ModMetaStorage {
    title: CString,
    author: CString,
    guid: CString,
    version: CString,
    desc: CString,
}

impl ModMetaStorage {
    pub fn new(
        title: &str,
        author: &str,
        guid: &str,
        version: &str,
        desc: &str,
    ) -> Self {
        Self {
            title: CString::new(title).unwrap(),
            author: CString::new(author).unwrap(),
            guid: CString::new(guid).unwrap(),
            version: CString::new(version).unwrap(),
            desc: CString::new(desc).unwrap(),
        }
    }

    pub fn as_raw(&self) -> JAPIModMetaRaw {
        JAPIModMetaRaw {
            name: self.title.as_ptr(),
            author: self.author.as_ptr(),
            guid: self.guid.as_ptr(),
            version: self.version.as_ptr(),
            description: self.desc.as_ptr(),
        }
    }
}

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

#[derive(Debug)]
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
    API.set(api).unwrap();

    Ok(())
}
