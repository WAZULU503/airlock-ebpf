#![no_std]
#![no_main]

mod vmlinux;

use aya_ebpf::{
    helpers::bpf_probe_read_kernel_str_bytes,
    macros::lsm,
    programs::LsmContext,
};

use aya_log_ebpf::info;

use crate::vmlinux::linux_binprm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    match try_enforce(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_enforce(ctx: LsmContext) -> Result<i32, i32> {
    let bprm: *const linux_binprm = ctx.arg(0);

    let filename_ptr =
        unsafe { (*bprm).filename as *const u8 };

    if filename_ptr.is_null() {
        return Ok(0);
    }

    let mut path_buf = [0u8; 256];

    let path = match unsafe {
        bpf_probe_read_kernel_str_bytes(
            filename_ptr,
            &mut path_buf,
        )
    } {
        Ok(path) => path,
        Err(_) => return Ok(0),
    };

    // strip trailing NULL
    let path = if let Some((&0, rest)) = path.split_last() {
        rest
    } else {
        path
    };

    if path == b"/usr/lib/cargo/bin/coreutils/ls" {
        info!(&ctx, "AIRLOCK DENY");

        return Ok(-1);
    }

    Ok(0)
}
