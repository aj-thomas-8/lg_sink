#[cfg(debug_assertions)]
use std::{io::Write, os::unix::net::UnixStream, path::Path, sync::LazyLock};

#[cfg(debug_assertions)]
const LOG_PATH: &str = "/tmp/rst-log.sock";

#[cfg(debug_assertions)]
static STREAM: LazyLock<UnixStream> = LazyLock::new(|| {
    UnixStream::connect(Path::new(LOG_PATH)).unwrap()
});

pub enum LogLevel {
    // Currently identical to Info; keeping for when log file creation is eventually 
    // implemented.
    Debug,
    Error,
    Info,
    Warn,
}

#[allow(unused)]
pub fn log(msg: &str, level: LogLevel) {
    #[cfg(not(debug_assertions))]
    {
        return;
    }

    #[cfg(debug_assertions)] 
    {
        let fmt_msg = match level {
            LogLevel::Debug => format!("[DBG]: {}\n", msg),
            LogLevel::Error => format!("[ERR]: {}\n", msg),
            LogLevel::Info => format!("[INF]: {}\n", msg),
            LogLevel::Warn => format!("[WRN]: {}\n", msg),
        };

        let _ = (&*STREAM).write_all(fmt_msg.as_bytes());
    }
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::log($msg, $crate::LogLevel::Info)
    };

    ($fmt:expr, $($arg:tt)*) => {{
        let formatted = format!($fmt, $($arg)*);
        $crate::log(&formatted, $crate::LogLevel::Info)
    }};
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::log($msg, $crate::LogLevel::Debug)
    };

    ($fmt:expr, $($arg:tt)*) => {{
        let formatted = format!($fmt, $($arg)*);
        $crate::log(&formatted, $crate::LogLevel::Debug)
    }};
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::log($msg, $crate::LogLevel::Error)
    };

    ($fmt:expr, $($arg:tt)*) => {{
        let formatted = format!($fmt, $($arg)*);
        $crate::log(&formatted, $crate::LogLevel::Error)
    }};
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        $crate::log($msg, $crate::LogLevel::Warn)
    };

    ($fmt:expr, $($arg:tt)*) => {{
        let formatted = format!($fmt, $($arg)*);
        $crate::log(&formatted, $crate::LogLevel::Warn)
    }};
}
