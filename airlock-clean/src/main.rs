use std::os::unix::fs::MetadataExt;

use aya::{
    maps::HashMap,
    programs::Lsm,
    Btf,
    Ebpf,
};

use airlock_clean_common::{
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

    match aya_log::EbpfLogger::init(&mut ebpf) {
        Ok(logger) => {
            let mut logger =
                tokio::io::unix::AsyncFd::with_interest(
                    logger,
                    tokio::io::Interest::READABLE,
                )?;

            tokio::task::spawn(async move {
                loop {
                    let mut guard =
                        logger.readable_mut().await.unwrap();

                    guard.get_inner_mut().flush();

                    guard.clear_ready();
                }
            });
        }

        Err(e) => {
            warn!(
                "failed to initialize eBPF logger: {}",
                e
            );
        }
    }

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

    signal::ctrl_c().await?;

    println!("Exiting...");

    Ok(())
}
