
#[cfg(target_os = "linux")]
pub mod epoll;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod event;

#[cfg(target_os = "linux")]
pub mod eventfd;

pub mod signal;

#[cfg(target_os = "linux")]
pub mod signalfd;

pub mod socket;

pub mod stat;

#[cfg(target_os = "linux")]
pub mod utsname;

pub mod wait;

pub mod mman;
