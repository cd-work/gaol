#[cfg(target_os = "freebsd")]
pub use crate::platform::freebsd::{ChildSandbox, Operation, Sandbox};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use crate::platform::linux::{ChildSandbox, Operation, Sandbox};
#[cfg(target_os = "macos")]
pub use crate::platform::macos::{ChildSandbox, Operation, Sandbox};
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd"
))]
pub use crate::platform::unix::process::{self, Process};

#[cfg(target_os = "freebsd")]
pub mod freebsd;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(any(target_os = "android", target_os = "linux", target_os = "macos", target_os = "freebsd"))]
pub mod unix;
