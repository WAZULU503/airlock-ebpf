use aya::{
    maps::HashMap,
    Btf,
    programs::Lsm,
};

use airlock_clean_common::{
    FileIdentity,
    PolicyEntry,
};

#[rustfmt::skip]
use log::{debug, warn};

use std::fs;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };

    let ret = unsafe {
        libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim)
    };

    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {ret}");
    }

    let mut ebpf = aya::Ebpf::load(
        aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/airlock-clean"
        ))
    )?;

    match aya_log::EbpfLogger::init(&mut ebpf) {
        Err(e) => {
            warn!("failed to initialize eBPF logger: {e}");
        }

        Ok(logger) => {
            let mut logger =
                tokio::io::unix::AsyncFd::with_interest(
                    logger,
                    tokio::io::Interest::READABLE
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
    }

    let btf = Btf::from_sys_fs()?;

    let program: &mut Lsm =
        ebpf.program_mut("bprm_check_security")
            .unwrap()
            .try_into()?;

    program.load("bprm_check_security", &btf)?;
    program.attach()?;

    let map_path = "/sys/fs/bpf/airlock_policy";

    if fs::metadata(map_path).is_ok() {
        let _ = fs::remove_file(map_path);
    }

    let mut policy_map: HashMap<
        _,
        FileIdentity,
        PolicyEntry
    > = HashMap::try_from(
        ebpf.map_mut("POLICY_MAP")
            .expect("POLICY_MAP not found")
    )?;

    policy_map.pin(map_path)?;

    println!("POLICY_MAP pinned: {}", map_path);

    let ctrl_c = signal::ctrl_c();

    println!("Waiting for Ctrl-C...");

    ctrl_c.await?;

    println!("Exiting...");

    Ok(())
}
