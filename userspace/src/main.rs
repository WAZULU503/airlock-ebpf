use aya::{Ebpf, Btf, maps::HashMap, programs::Lsm};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    // Load eBPF object
    let mut bpf = Ebpf::load_file("target/bpfel-unknown-none/release/airlock-ebpf")?;

    // Load kernel BTF
    let btf = Btf::from_sys_fs()?;

    // Load + attach LSM
    let program: &mut Lsm = bpf.program_mut("bprm_check_security")
        .ok_or(anyhow!("program not found"))?
        .try_into()?;

    program.load("bprm_check_security", &btf)?;
    program.attach()?;

    println!("LSM loaded");

    // Get BLOCKLIST map
    let mut blocklist: HashMap<_, [u8; 256], u8> =
        HashMap::try_from(
            bpf.map_mut("BLOCKLIST").ok_or(anyhow!("map not found"))?
        )?;

    // RULE 1
    let mut key = [0u8; 256];
    let path = b"/usr/bin/whoami";
    key[..path.len()].copy_from_slice(path);
    blocklist.insert(key, 1u8, 0)?;

    // RULE 2
    let mut key2 = [0u8; 256];
    let path2 = b"/usr/bin/id";
    key2[..path2.len()].copy_from_slice(path2);
    blocklist.insert(key2, 1u8, 0)?;

    println!("Policy loaded: whoami + id → DENY");

    // KEEP PROCESS ALIVE
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
