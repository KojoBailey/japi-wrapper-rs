use std::ffi::CString;
use std::os::raw::c_char;

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
        desc: $desc:expr
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
            storage.as_raw()
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

#[link(name = "JAPI")]
unsafe extern "C" {
    fn JAPI_LogMessage(level: i32, msg: *const c_char);
}

pub fn log(level: LogLevel, msg: &str) {
    let c_str = CString::new(msg).unwrap();
    unsafe {
        JAPI_LogMessage(level as i32, c_str.as_ptr());
    }
}
