#![no_std]
#![no_main]

use aya_ebpf::{
    macros::lsm,
    programs::LsmContext,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(_ctx: LsmContext) -> i32 {
    -1
}
