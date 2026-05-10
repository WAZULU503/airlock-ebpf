use aya::{
    include_bytes_aligned,
    maps::RingBuf,
    util::online_cpus,
    Ebpf,
    Btf,
};

use aya::programs::Lsm;

use tokio::io::unix::AsyncFd;

use airlock_clean_common::ExecEvent;

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
                    "exec dev={} ino={}",
                    evt.dev,
                    evt.ino
                );
            }
        }

        guard.clear_ready();
    }
}
