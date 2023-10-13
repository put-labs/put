#![cfg(feature = "program")]

pub use put_program::log::*;

#[macro_export]
#[deprecated(
    since = "1.4.3",
    note = "Please use `put_program::log::info` instead"
)]
macro_rules! info {
    ($msg:expr) => {
        $crate::log::put_log($msg)
    };
}
