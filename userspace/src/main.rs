use aya::{
    Ebpf, Btf,
    maps::{HashMap, Map, MapData},
    programs::Lsm,
};
use anyhow::{Result, anyhow};
use std::{env, path::Path};

const PIN_PATH: &str = "/sys/fs/bpf/airlock_blocklist";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("  airlock-user block /path");
        println!("  airlock-user unblock /path");
        return Ok(());
    }

    let command = &args[1];
    let target_path = &args[2];

    // === BUILD KEY ===
    let mut key = [0u8; 256];
    let bytes = target_path.as_bytes();

    if bytes.len() >= 256 {
        return Err(anyhow!("path too long"));
    }

    key[..bytes.len()].copy_from_slice(bytes);

    match command.as_str() {

        // =========================
        // DAEMON (BLOCK)
        // =========================
        "block" => {
            let mut bpf = Ebpf::load_file(
                "target/bpfel-unknown-none/release/airlock-ebpf"
            )?;

            let btf = Btf::from_sys_fs()?;

            let program: &mut Lsm = bpf
                .program_mut("bprm_check_security")
                .ok_or(anyhow!("program not found"))?
                .try_into()?;

            program.load("bprm_check_security", &btf)?;
            program.attach()?;

            println!("LSM loaded");

            let blocklist: HashMap<_, [u8; 256], u8> =
                HashMap::try_from(
                    bpf.map_mut("BLOCKLIST")
                        .ok_or(anyhow!("map not found"))?
                )?;

            // PIN (consumes blocklist)
            if !Path::new(PIN_PATH).exists() {
                blocklist.pin(PIN_PATH)?;
                println!("Map pinned at {}", PIN_PATH);
            }

            // 🔑 REOPEN PINNED MAP (NEW HANDLE)
            let map_data = MapData::from_pin(PIN_PATH)?;
            let map = Map::from_map_data(map_data)?;
            let mut blocklist: HashMap<_, [u8; 256], u8> =
                HashMap::try_from(map)?;

            blocklist.insert(key, 1u8, 0)?;
            println!("Blocked: {}", target_path);

            loop {
                std::thread::sleep(std::time::Duration::from_secs(60));
            }
        }

        // =========================
        // CLI (UNBLOCK)
        // =========================
        "unblock" => {
            if !Path::new(PIN_PATH).exists() {
                return Err(anyhow!("No pinned map — daemon not running"));
            }

            let map_data = MapData::from_pin(PIN_PATH)?;
            let map = Map::from_map_data(map_data)?;
            let mut blocklist: HashMap<_, [u8; 256], u8> =
                HashMap::try_from(map)?;

            blocklist.remove(&key)?;
            println!("Unblocked: {}", target_path);
        }

        _ => {
            println!("Invalid command");
        }
    }

    Ok(())
}
