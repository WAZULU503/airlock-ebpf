#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{lsm, map},
    maps::HashMap,
    programs::LsmContext,
};

#[map(name = "BLOCKLIST")]
static mut BLOCKLIST: HashMap<[u8; 256], u8> =
    HashMap::<[u8; 256], u8>::with_max_entries(1024, 0);

#[lsm]
pub fn bprm_check_security(_ctx: LsmContext) -> i32 {
    unsafe {
        let mut key = [0u8; 256];
        key[0] = 1;

        if BLOCKLIST.get(&key).is_some() {
            return -1;
        }
    }

    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
