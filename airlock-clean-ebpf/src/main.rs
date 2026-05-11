#![no_std]
#![no_main]

mod vmlinux;

use airlock_clean_common::{
    ACTION_ALLOW,
    ACTION_DENY,
    ACTION_MISS,
    ExecutionEvent,
    FileIdentity,
    PolicyEntry,
};

use aya_ebpf::{
    macros::{
        lsm,
        map,
    },
    maps::{
        HashMap,
        RingBuf,
    },
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

#[map(name = "EVENTS")]
static mut EVENTS: RingBuf =
    RingBuf::with_byte_size(256 * 1024, 0);

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

    let action = unsafe {
        match POLICY_MAP.get(&identity) {
            Some(entry) => entry.action,
            None => ACTION_MISS,
        }
    };

    let event = ExecutionEvent {
        dev,
        ino,
        action,
        _pad: 0,
    };

    unsafe {
        EVENTS.output::<ExecutionEvent>(&event, 0);
    }

    match action {
        ACTION_ALLOW => {
            Ok(0)
        }

        ACTION_DENY => {
            Err(-1)
        }

        ACTION_MISS => {
            Ok(0)
        }

        _ => {
            Ok(0)
        }
    }
}
