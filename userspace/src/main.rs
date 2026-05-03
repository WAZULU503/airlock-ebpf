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
    use aya::maps::HashMap;
    let mut blocklist: HashMap<_, [u8; 256], u8> = HashMap::try_from(bpf.map_mut("BLOCKLIST").ok_or(anyhow!("map not found"))?)?;
    let mut key = [0u8; 256];
    let path = b"/usr/bin/whoami";
    key[..path.len()].copy_from_slice(path);
    blocklist.insert(key, 1u8, 0)?;
    println!("Policy loaded: /usr/bin/whoami → DENY");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
