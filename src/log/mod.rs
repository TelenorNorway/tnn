pub mod internals;

#[macro_export]
macro_rules! debug {($($arg:tt)+) => ($crate::log!($crate::log::internals::Level::Debug, $($arg)+))}

#[macro_export]
macro_rules! info {($($arg:tt)+) => ($crate::log!($crate::log::internals::Level::Info, $($arg)+))}

#[macro_export]
macro_rules! warn {($($arg:tt)+) => ($crate::log!($crate::log::internals::Level::Warning, $($arg)+))}

#[macro_export]
macro_rules! critical {($($arg:tt)+) => ($crate::log!($crate::log::internals::Level::Critical, $($arg)+))}

#[macro_export]
macro_rules! log {
	($lvl:expr, $($arg:tt)+) => ($crate::log::internals::log($lvl, $crate::log::internals::format_args!($($arg)+).to_string()));
}
