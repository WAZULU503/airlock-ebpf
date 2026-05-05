#![no_std]
#![no_main]

use aya_ebpf::{
    macros::lsm,
    programs::LsmContext,
};

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(_ctx: LsmContext) -> i32 {
    // correct LSM denial
    -13
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
