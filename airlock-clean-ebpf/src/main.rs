#![no_std]
#![no_main]

mod vmlinux;

use aya_ebpf::{
    macros::{lsm, map},
    maps::{HashMap, RingBuf},
    programs::LsmContext,
};

use airlock_clean_common::{
    ACTION_ALLOW, ACTION_DENY,
    ExecEvent,
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
static POLICY_MAP:
    HashMap<FileIdentity, PolicyEntry> =
        HashMap::with_max_entries(1024, 0);

#[map]
static EVENTS: RingBuf =
    RingBuf::with_byte_size(4096, 0);

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    unsafe {
        observe(&ctx);
    }

    0
}

unsafe fn observe(ctx: &LsmContext) {
    let bprm: *const linux_binprm =
        ctx.arg(0);

    let file_ptr: *mut file =
        (*bprm).file;

    if file_ptr.is_null() {
        return;
    }

    let f_path: path =
        (*file_ptr).__bindgen_anon_1.f_path;

    let dentry_ptr: *mut dentry =
        f_path.dentry;

    if dentry_ptr.is_null() {
        return;
    }

    let inode_ptr: *mut inode =
        (*dentry_ptr).d_inode;

    if inode_ptr.is_null() {
        return;
    }

    let ino: u64 =
        (*inode_ptr).i_ino as u64;

    let sb_ptr: *mut super_block =
        (*inode_ptr).i_sb;

    if sb_ptr.is_null() {
        return;
    }
    let dev: u64 =
        (*sb_ptr).s_dev as u64;

    let identity = FileIdentity {
        dev: 0,
        ino,
    };

    let action: u32 =
        match POLICY_MAP.get(&identity) {
            Some(policy) => policy.action,
            None => ACTION_ALLOW,
        };

    if let Some(mut entry) =
        EVENTS.reserve::<ExecEvent>(0)
    {
        let evt =
            entry.as_mut_ptr();

        (*evt).dev = dev;
        (*evt).ino = ino;
        (*evt).action = action;
        (*evt).reserved = 0;

        entry.submit(0);
    }
}
