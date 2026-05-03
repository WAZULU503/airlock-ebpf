#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{lsm, map},
    maps::{HashMap, PerCpuArray},
    programs::LsmContext,
    helpers::{bpf_probe_read_kernel, bpf_probe_read_kernel_str_bytes},
};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[map]
static BLOCKLIST: HashMap<[u8; 256], u8> = HashMap::with_max_entries(128, 0);

#[map]
static PATH_BUF: PerCpuArray<[u8; 256]> = PerCpuArray::with_max_entries(1, 0);

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    unsafe { try_check(&ctx).unwrap_or(0) }
}

unsafe fn try_check(ctx: &LsmContext) -> Option<i32> {
    let bprm = ctx.arg::<*const u8>(0);
    if bprm.is_null() {
        return Some(0);
    }

    let filename_ptr: *const u8 =
        bpf_probe_read_kernel(bprm.add(96) as *const *const u8).ok()?;

    let buf = &mut *PATH_BUF.get_ptr_mut(0)?;
    bpf_probe_read_kernel_str_bytes(filename_ptr, buf).ok()?;

    let mut key = [0u8; 256];
    let mut i = 0;
    while i < 256 && buf[i] != 0 {
        key[i] = buf[i];
        i += 1;
    }

    if BLOCKLIST.get(&key).is_some() {
        return Some(-1);
    }

    Some(0)
}
