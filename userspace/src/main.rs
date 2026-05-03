use aya::{Ebpf, programs::Lsm, Btf};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let mut bpf = Ebpf::load_file("target/bpfel-unknown-none/release/airlock-ebpf")?;

    let program = bpf.program_mut("bprm_check_security")
        .ok_or(anyhow!("program not found"))?;
    let program: &mut Lsm = program.try_into()?;

    let btf = Btf::from_sys_fs()?;
    program.load("bprm_check_security", &btf)?;
    program.attach()?;

    println!("LSM loaded");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
