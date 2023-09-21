pub mod internals;

#[macro_export]
macro_rules! debug {($($arg:tt)+) => ($crate::log!($crate::internals::Level::Debug, $($arg)+))}

#[macro_export]
macro_rules! info {($($arg:tt)+) => ($crate::log!($crate::internals::Level::Info, $($arg)+))}

#[macro_export]
macro_rules! warn {($($arg:tt)+) => ($crate::log!($crate::internals::Level::Warning, $($arg)+))}

#[macro_export]
macro_rules! critical {($($arg:tt)+) => ($crate::log!($crate::internals::Level::Critical, $($arg)+))}

#[macro_export]
macro_rules! log {
	($lvl:expr, $($arg:tt)+) => ($crate::internals::log($lvl, $crate::internals::format_args!($($arg)+).to_string()));
}
