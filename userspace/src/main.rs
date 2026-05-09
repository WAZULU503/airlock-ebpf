use aya::{Ebpf, programs::Lsm, Btf, maps::HashMap};
use serde::Deserialize;
use std::{fs, thread, time::Duration, path::Path};

const RULES_PATH: &str = "/var/lib/airlock/rules.json";

#[derive(Deserialize)]
struct Rules {
    version: u8,
    rules: Vec<String>,
}

fn load_rules() -> Result<Vec<String>, anyhow::Error> {
    let data = fs::read_to_string(RULES_PATH)?;
    let parsed: Rules = serde_json::from_str(&data)?;

    let mut normalized = Vec::new();
    for r in parsed.rules {
        let p = Path::new(&r).canonicalize()?;
        normalized.push(p.to_string_lossy().to_string());
    }

    Ok(normalized)
}

fn main() -> Result<(), anyhow::Error> {
    let rules = load_rules()?;

    // 🔥 LOAD REAL BPF ELF (Aya-generated path)
    let mut bpf = Ebpf::load_file(
        "target/bpfel-unknown-none/release/libairlock_ebpf.so"
    )?;

    let program: &mut Lsm = bpf.program_mut("bprm_check_security")
        .ok_or(anyhow::anyhow!("program not found"))?
        .try_into()?;

    let btf = Btf::from_sys_fs()?;
    program.load("bprm_check_security", &btf)?;
    program.attach()?;

    let map_ref = bpf.map_mut("BLOCKLIST")
        .ok_or(anyhow::anyhow!("FAIL: map not found"))?;

    let mut map: HashMap<_, [u8;256], u8> =
        HashMap::try_from(map_ref)?;

    for rule in rules {
        let mut key = [0u8; 256];
        let bytes = rule.as_bytes();
        let len = bytes.len().min(255);
        key[..len].copy_from_slice(&bytes[..len]);
        map.insert(key, 1, 0)?;
    }

    println!("LSM attached");

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
