#![no_std]
#![no_main]

mod vmlinux;

use core::ffi::CStr;

use airlock_clean_common::{
    ACTION_DENY,
    FileIdentity,
    PolicyEntry,
};

use aya_ebpf::{
    helpers::bpf_printk,
    macros::{
        lsm,
        map,
    },
    maps::HashMap,
    programs::LsmContext,
};

use crate::vmlinux::{
    dentry,
    file,
    inode,
    linux_binprm,
    path,
    super_block,
};

#[map(name = "POLICY_MAP")]
static mut POLICY_MAP: HashMap<FileIdentity, PolicyEntry> =
    HashMap::<FileIdentity, PolicyEntry>::with_max_entries(1024, 0);

const LOOKUP_LOG: &CStr =
    c"AIRLOCK lookup dev=%llu ino=%llu";

const DENY_LOG: &CStr =
    c"AIRLOCK DENY dev=%llu ino=%llu";

const MISS_LOG: &CStr =
    c"AIRLOCK MISS dev=%llu ino=%llu";

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
    let bprm: *const linux_binprm =
        ctx.arg(0);

    let file_ptr: *mut file =
        unsafe { (*bprm).file };

    if file_ptr.is_null() {
        return Ok(0);
    }

    let f_path: path =
        unsafe { (*file_ptr).__bindgen_anon_1.f_path };

    let dentry_ptr: *mut dentry =
        f_path.dentry;

    if dentry_ptr.is_null() {
        return Ok(0);
    }

    let inode_ptr: *mut inode =
        unsafe { (*dentry_ptr).d_inode };

    if inode_ptr.is_null() {
        return Ok(0);
    }

    let ino =
        unsafe { (*inode_ptr).i_ino as u64 };

    let sb_ptr: *mut super_block =
        unsafe { (*inode_ptr).i_sb };

    if sb_ptr.is_null() {
        return Ok(0);
    }

    let dev =
        unsafe { (*sb_ptr).s_dev as u64 };

    let identity = FileIdentity {
        dev,
        ino,
    };

    unsafe {
        bpf_printk!(
            LOOKUP_LOG,
            dev,
            ino
        );
    }

    let action = unsafe {
        match POLICY_MAP.get(&identity) {
            Some(entry) => entry.action,

            None => {
                bpf_printk!(
                    MISS_LOG,
                    dev,
                    ino
                );

                0
            }
        }
    };

    if action == ACTION_DENY {
        unsafe {
            bpf_printk!(
                DENY_LOG,
                dev,
                ino
            );
        }

        return Ok(-1);
    }

    Ok(0)
}
