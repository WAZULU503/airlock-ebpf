#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::bpf_get_current_comm,
    macros::lsm,
    programs::LsmContext,
    EbpfContext,
};

use aya_log_ebpf::info;

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
    let comm = match bpf_get_current_comm() {
        Ok(comm) => comm,
        Err(_) => return Ok(0),
    };

    // STRICT command-name enforcement
    // safer than broad path matching during stabilization

    if comm.starts_with(b"ls") {
        info!(
            &ctx,
            "AIRLOCK DENY pid={} comm=ls",
            ctx.pid(),
        );

        return Ok(-1);
    }

    Ok(0)
}
