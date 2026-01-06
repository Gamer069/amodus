#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogKind::Trace, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogKind::Debug, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogKind::Info, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogKind::Warn, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogKind::Error, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::info!("")
    };
    ($($arg:tt)*) => {
        $crate::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::error!("")
    };
    ($($arg:tt)*) => {
        $crate::error!($($arg)*)
    };
}

#[repr(u8)]
#[derive(Debug)]
pub enum LogKind {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

pub fn log(kind: LogKind, txt: String) {
    unsafe {
        amodus_log(kind as u8, txt.as_ptr(), txt.len());
    }
}

unsafe extern "C" {
    fn amodus_log(kind: u8, ptr: *const u8, len: usize);
}
