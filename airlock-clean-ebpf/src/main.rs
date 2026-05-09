#![no_std]
#![no_main]

mod vmlinux;

use aya_ebpf::{
    macros::{lsm, map},
    maps::HashMap,
    programs::LsmContext,
};

use airlock_clean_common::{
    ACTION_ALLOW,
    FileIdentity,
    PolicyEntry,
};

use crate::vmlinux::{
    dentry,
    file,
    inode,
    linux_binprm,
    path,
    super_block,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[map]
static POLICY_MAP: HashMap<FileIdentity, PolicyEntry> =
    HashMap::with_max_entries(1024, 0);

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    match try_enforce(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_enforce(ctx: LsmContext) -> Result<i32, i32> {
    let bprm: *const linux_binprm = ctx.arg(0);

    let file_ptr: *mut file = unsafe { (*bprm).file };

    if file_ptr.is_null() {
        return Ok(0);
    }

    // bindgen wrapped f_path in anonymous union
    let f_path: path =
        unsafe { (*file_ptr).__bindgen_anon_1.f_path };

    let dentry_ptr: *mut dentry = f_path.dentry;

    if dentry_ptr.is_null() {
        return Ok(0);
    }

    let inode_ptr: *mut inode =
        unsafe { (*dentry_ptr).d_inode };

    if inode_ptr.is_null() {
        return Ok(0);
    }

    let ino = unsafe { (*inode_ptr).i_ino as u64 };

    let sb_ptr: *mut super_block =
        unsafe { (*inode_ptr).i_sb };

    if sb_ptr.is_null() {
        return Ok(0);
    }

    let dev = unsafe { (*sb_ptr).s_dev };

    unsafe {
        core::ptr::read_volatile(&dev);
    }

    // inode-backed deny test
    if ino == 2621951 {
        return Ok(-1);
    }

    Ok(0)
}
