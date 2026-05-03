#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::bpf_probe_read_kernel,
    macros::lsm,
    programs::LsmContext,
};
use core::panic::PanicInfo;

const FILENAME_OFFSET: usize = 96;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lsm]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    unsafe {
        let bprm: *const u8 = ctx.arg(0);

        let filename_ptr: *const u8 =
            bpf_probe_read_kernel(bprm.add(FILENAME_OFFSET) as *const *const u8)
                .unwrap_or(core::ptr::null());

        if filename_ptr.is_null() {
            return 0;
        }

        let buf: [u8; 16] =
            bpf_probe_read_kernel(filename_ptr as *const [u8; 16])
                .unwrap_or([0u8; 16]);

        if buf.starts_with(b"/bin/ls") {
            return -1;
        }

        0
    }
}
