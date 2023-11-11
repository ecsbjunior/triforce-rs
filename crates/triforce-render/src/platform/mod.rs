#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
pub mod platform;

pub use platform::*;
