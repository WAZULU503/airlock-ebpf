use std::{
    mem,
    os::unix::fs::MetadataExt,
    slice,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use aya::{
    maps::{
        HashMap,
        RingBuf,
    },
    programs::Lsm,
    util::online_cpus,
    Btf,
    Ebpf,
};

use airlock_clean_common::{
    ACTION_ALLOW,
    ACTION_DENY,
    ACTION_MISS,
    ExecutionEvent,
    FileIdentity,
    PolicyEntry,
};

#[rustfmt::skip]
use log::{debug, warn};

use tokio::signal;

fn canonical_dev(dev: u64) -> u64 {
    let major =
        ((dev >> 8) & 0xfff) |
        ((dev >> 32) & !0xfff);

    let minor =
        (dev & 0xff) |
        ((dev >> 12) & !0xff);

    (major << 20) | minor
}

fn action_name(action: u32) -> &'static str {
    match action {
        ACTION_ALLOW => "ALLOW",
        ACTION_DENY => "DENY",
        ACTION_MISS => "MISS",
        _ => "UNKNOWN",
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };

    let ret = unsafe {
        libc::setrlimit(
            libc::RLIMIT_MEMLOCK,
            &rlim,
        )
    };

    if ret != 0 {
        debug!(
            "failed to increase rlimit: {}",
            ret
        );
    }

    let mut ebpf = Ebpf::load(
        aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/airlock-clean-ebpf"
        ))
    )?;

    let mut policy_map: HashMap<
        _,
        FileIdentity,
        PolicyEntry,
    > = HashMap::try_from(
        ebpf.map_mut("POLICY_MAP").unwrap()
    )?;

    let deny_meta =
        std::fs::metadata("/usr/bin/ping")?;

    let deny_identity = FileIdentity {
        dev: canonical_dev(deny_meta.dev()),
        ino: deny_meta.ino(),
    };

    println!(
        "DENY /usr/bin/ping dev={} ino={}",
        deny_identity.dev,
        deny_identity.ino
    );

    policy_map.insert(
        deny_identity,
        PolicyEntry::deny(),
        0,
    )?;

    let allow_meta =
        std::fs::metadata("/usr/bin/dash")?;

    let allow_identity = FileIdentity {
        dev: canonical_dev(allow_meta.dev()),
        ino: allow_meta.ino(),
    };

    println!(
        "ALLOW /usr/bin/dash dev={} ino={}",
        allow_identity.dev,
        allow_identity.ino
    );

    policy_map.insert(
        allow_identity,
        PolicyEntry::allow(),
        0,
    )?;

    let btf = Btf::from_sys_fs()?;

    let program: &mut Lsm =
        ebpf.program_mut("bprm_check_security")
            .unwrap()
            .try_into()?;

    program.load(
        "bprm_check_security",
        &btf,
    )?;

    program.attach()?;

    println!("AIRLOCK loaded");
    println!("Waiting for Ctrl-C...");

    let mut ring =
        RingBuf::try_from(
            ebpf.map_mut("EVENTS").unwrap()
        )?;


    loop {
        while let Some(item) = ring.next() {
                let bytes = item.as_ref();

                if bytes.len()
                    != mem::size_of::<ExecutionEvent>()
                {
                    continue;
                }

                let event: ExecutionEvent = unsafe {
                    core::ptr::read_unaligned(
                        bytes.as_ptr()
                            as *const ExecutionEvent
                    )
                };

                let ts =
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                println!(
                    "{{\"v\":1,\"ts\":{},\"dev\":{},\"ino\":{},\"action\":\"{}\"}}",
                    ts,
                    event.dev,
                    event.ino,
                    action_name(event.action),
                );
                    }
    }



    signal::ctrl_c().await?;

    println!("Exiting...");

    Ok(())
}
