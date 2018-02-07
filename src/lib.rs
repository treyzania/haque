extern crate libc;

#[cfg(os = "unix")]
pub mod fob;

#[cfg(os = "unix")]
pub mod man;
