#[macro_export]
macro_rules! logI {
    ($tag:expr, $($arg:tt)*) => {
        log::info!("{} : {}", $tag, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! logD {
    ($tag:expr, $($arg:tt)*) => {
        log::debug!("{} : {}", $tag, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! logE {
    ($tag:expr, $($arg:tt)*) => {
        log::error!("{} : {}", $tag, format!($($arg)*));
    };
} 