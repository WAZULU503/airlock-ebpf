#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::result::Result;

use aya_ebpf::{
    helpers::{bpf_probe_read_kernel, bpf_probe_read_kernel_str_bytes, bpf_printk},
    macros::lsm,
    programs::LsmContext,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lsm]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    match unsafe { try_block(ctx) } {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

unsafe fn try_block(ctx: LsmContext) -> Result<i32, i32> {
    let bprm: *const u8 = ctx.arg(0);

    // OFFSET (your working one = 104)
    let filename_ptr: *const u8 =
        bpf_probe_read_kernel(bprm.add(104) as *const *const u8)
            .map_err(|_| 0)?;

    let mut buf = [0u8; 256];

    if bpf_probe_read_kernel_str_bytes(filename_ptr, &mut buf).is_ok() {

        // DEBUG PRINT (first chars)
        bpf_printk!(
            b"PATH: %c%c%c%c%c%c\n",
            buf[0], buf[1], buf[2],
            buf[3], buf[4], buf[5]
        );

        // FIND STRING LENGTH
        let mut len = 0usize;
        for i in 0..255usize {
            if buf[i] == 0 {
                len = i;
                break;
            }
        }

        // MATCH END == "whoami"
        if len >= 6 {
            if buf[len - 6] == b'w' &&
               buf[len - 5] == b'h' &&
               buf[len - 4] == b'o' &&
               buf[len - 3] == b'a' &&
               buf[len - 2] == b'm' &&
               buf[len - 1] == b'i' {
                return Ok(-1);
            }
        }
    }

    Ok(0)
}
