use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "JAPI")]
unsafe extern "C" {
    fn JAPI_LogMessage(level: i32, msg: *const c_char);
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

pub fn log(level: LogLevel, msg: &str) {
    let c_str = CString::new(msg).unwrap();
    unsafe {
        JAPI_LogMessage(level as i32, c_str.as_ptr());
    }
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Fatal, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Error, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Warn, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Debug, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::Trace, &format!($($arg)*))
    };
}
