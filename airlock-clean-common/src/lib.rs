#![no_std]

#[cfg(feature = "user")]
use aya::Pod;

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq
)]
pub struct FileIdentity {
    pub dev: u64,
    pub ino: u64,
}

// Compile-time ABI guard
const _: [(); 16] = [(); core::mem::size_of::<FileIdentity>()];

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq
)]
pub struct PolicyEntry {
    pub action: u32,
    pub reserved: u32,
}

// Compile-time ABI guard
const _: [(); 8] = [(); core::mem::size_of::<PolicyEntry>()];

pub const ACTION_ALLOW: u32 = 0;
pub const ACTION_DENY: u32 = 1;

#[cfg(feature = "user")]
unsafe impl Pod for FileIdentity {}

#[cfg(feature = "user")]
unsafe impl Pod for PolicyEntry {}
