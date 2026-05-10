use aya::{
    include_bytes_aligned,
    maps::{HashMap, RingBuf},
    util::online_cpus,
    Ebpf,
    Btf,
};

use std::fs;

use airlock_clean_common::{
    ACTION_DENY,
    ExecEvent,
    FileIdentity,
    PolicyEntry,
};

use aya::programs::Lsm;

use tokio::io::unix::AsyncFd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ebpf = Ebpf::load(include_bytes_aligned!(
        concat!(
            env!("OUT_DIR"),
            "/airlock-clean"
        )
    ))?;

    let btf = Btf::from_sys_fs()?;

    let program: &mut Lsm =
        ebpf.program_mut("bprm_check_security")
            .unwrap()
            .try_into()?;

    program.load(
        "bprm_check_security",
        &btf
    )?;

    program.attach()?;

    println!("[airlock] LSM attached");

    let metadata =
        fs::metadata("/usr/bin/id")?;

    use std::os::linux::fs::MetadataExt;

    let identity = FileIdentity {
        dev: 0,
        ino: metadata.st_ino(),
    };

    let policy = PolicyEntry {
        action: ACTION_DENY,
        reserved: 0,
    };

    let mut policy_map =
        HashMap::<_, FileIdentity, PolicyEntry>::try_from(
            ebpf.map_mut("POLICY_MAP").unwrap()
        )?;

    policy_map.insert(identity, policy, 0)?;

    println!(
        "[airlock] inserted DENY rule for /usr/bin/id dev={} ino={}",
        identity.dev,
        identity.ino,
    );

    let ring =
        RingBuf::try_from(
            ebpf.map_mut("EVENTS").unwrap()
        )?;

    let mut async_fd =
        AsyncFd::new(ring)?;

    println!("[airlock] observing exec events...");

    loop {
        let mut guard =
            async_fd.readable_mut().await?;

        let rb =
            guard.get_inner_mut();

        while let Some(item) = rb.next() {
            if item.len()
                == core::mem::size_of::<ExecEvent>()
            {
                let evt = unsafe {
                    &*(item.as_ptr()
                        as *const ExecEvent)
                };

                println!(
                    "exec dev={} ino={} action={}",
                    evt.dev,
                    evt.ino,
                    match evt.action {
                        1 => "DENY",
                        _ => "ALLOW",
                    }
                );
            }
        }

        guard.clear_ready();
    }
}
