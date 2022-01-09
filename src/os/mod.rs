mod shared;
pub use self::shared::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(target_os="windows"))]
mod unix;
#[cfg(not(target_os="windows"))]
use self::unix::*;
