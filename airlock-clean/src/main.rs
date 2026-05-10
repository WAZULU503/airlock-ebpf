use aya::{
    include_bytes_aligned,
    Ebpf,
    Btf,
};

use aya::programs::Lsm;

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

    println!("LSM attached");
    println!("Waiting for Ctrl-C...");

    tokio::signal::ctrl_c().await?;

    Ok(())
}
