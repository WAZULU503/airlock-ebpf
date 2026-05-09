use aya::{Ebpf, maps::{HashMap, MapData}};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const RULES_PATH: &str = "/var/lib/airlock/rules.json";
const TMP_PATH: &str = "/var/lib/airlock/rules.json.tmp";

fn write_atomic(data: &str) -> Result<(), anyhow::Error> {
    let mut file = File::create(TMP_PATH)?;
    file.write_all(data.as_bytes())?;
    file.sync_all()?;

    fs::rename(TMP_PATH, RULES_PATH)?;

    let dir = File::open("/var/lib/airlock")?;
    dir.sync_all()?;

    Ok(())
}

fn load_rules() -> Result<Vec<String>, anyhow::Error> {
    let data = fs::read_to_string(RULES_PATH)?;

    let parsed: serde_json::Value =
        serde_json::from_str(&data)?;

    let rules = parsed["rules"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("invalid rules"))?;

    rules.iter()
        .map(|r| {
            r.as_str()
                .ok_or_else(|| anyhow::anyhow!("invalid rule"))
                .map(|s| s.to_string())
        })
        .collect()
}

fn save_rules(rules: &[String]) -> Result<(), anyhow::Error> {
    let data = serde_json::json!({
        "version": 1,
        "rules": rules
    });

    write_atomic(&data.to_string())
}

fn key_from_path(path: &str) -> [u8; 256] {
    let mut key = [0u8; 256];

    let bytes = path.as_bytes();
    let len = bytes.len().min(255);

    key[..len].copy_from_slice(&bytes[..len]);

    key
}

fn load_map<'a>(
    bpf: &'a mut Ebpf
) -> Result<HashMap<&'a mut MapData, [u8; 256], u8>, anyhow::Error> {
    let map_ref = bpf.map_mut("BLOCKLIST")
        .ok_or(anyhow::anyhow!("map not found"))?;

    Ok(HashMap::try_from(map_ref)?)
}

fn usage() -> anyhow::Error {
    anyhow::anyhow!(
        "Usage: airlock-ctl <add|remove|list> [path]"
    )
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(usage());
    }

    let mut rules = load_rules()?;

    let mut bpf = Ebpf::load_file(
        "target/bpfel-unknown-none/release/libairlock_ebpf.so"
    )?;

    let mut map = load_map(&mut bpf)?;

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                return Err(anyhow::anyhow!(
                    "add requires path"
                ));
            }

            let p = Path::new(&args[2]).canonicalize()?;
            let path = p.to_string_lossy().to_string();

            if !rules.contains(&path) {
                rules.push(path.clone());

                save_rules(&rules)?;

                map.insert(
                    key_from_path(&path),
                    1,
                    0
                )?;

                println!("[AIRLOCK] ADD OK: {}", path);
            }
        }

        "remove" => {
            if args.len() < 3 {
                return Err(anyhow::anyhow!(
                    "remove requires path"
                ));
            }

            let p = Path::new(&args[2]).canonicalize()?;
            let path = p.to_string_lossy().to_string();

            rules.retain(|r| r != &path);

            save_rules(&rules)?;

            let _ = map.remove(&key_from_path(&path));

            println!("[AIRLOCK] REMOVE OK: {}", path);
        }

        "list" => {
            for entry in map.iter() {
                let (key, _) = entry?;

                let s = String::from_utf8_lossy(&key);

                println!("{}", s.trim_matches('\0'));
            }
        }

        _ => return Err(usage()),
    }

    Ok(())
}
