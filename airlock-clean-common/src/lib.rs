#![no_std]

#[cfg(feature = "user")]
use aya::Pod;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FileIdentity {
    pub dev: u64,
    pub ino: u64,
}

const _: [(); 16] =
    [(); core::mem::size_of::<FileIdentity>()];

#[cfg(feature = "user")]
unsafe impl Pod for FileIdentity {}

pub const ACTION_ALLOW: u32 = 1;
pub const ACTION_DENY: u32 = 2;
pub const ACTION_MISS: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PolicyEntry {
    pub action: u32,
    pub _reserved: u32,
}

const _: [(); 8] =
    [(); core::mem::size_of::<PolicyEntry>()];

#[cfg(feature = "user")]
unsafe impl Pod for PolicyEntry {}

impl PolicyEntry {
    pub const fn allow() -> Self {
        Self {
            action: ACTION_ALLOW,
            _reserved: 0,
        }
    }

    pub const fn deny() -> Self {
        Self {
            action: ACTION_DENY,
            _reserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExecutionEvent {
    pub dev: u64,
    pub ino: u64,
    pub action: u32,
    pub _pad: u32,
}

const _: [(); 24] =
    [(); core::mem::size_of::<ExecutionEvent>()];

#[cfg(feature = "user")]
unsafe impl Pod for ExecutionEvent {}
