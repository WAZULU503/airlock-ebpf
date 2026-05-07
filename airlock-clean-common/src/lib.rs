#![no_std]

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FileIdentity {
    pub dev: u64,
    pub ino: u64,
}

// Compile-time ABI guard
const _: [(); 16] = [(); core::mem::size_of::<FileIdentity>()];

pub const ACTION_ALLOW: u32 = 0;
pub const ACTION_DENY: u32 = 1;
