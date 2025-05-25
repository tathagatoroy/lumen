#[macro_export]
macro_rules! tagInfo {
    ($tag:expr, $($arg:tt)*) => {
        log::info!("{} : {}", $tag, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! tagDebug {
    ($tag:expr, $($arg:tt)*) => {
        log::debug!("{} : {}", $tag, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! tagError {
    ($tag:expr, $($arg:tt)*) => {
        log::error!("{} : {}", $tag, format!($($arg)*));
    };
} 