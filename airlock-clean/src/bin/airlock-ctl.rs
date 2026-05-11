use aya::maps::{HashMap, MapData};

use airlock_clean_common::{
    ACTION_DENY,
    FileIdentity,
    PolicyEntry,
};

use std::os::unix::fs::MetadataExt;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> =
        std::env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "usage: airlock-ctl --deny|--allow <path>"
        );

        std::process::exit(1);
    }

    let action = &args[1];
    let path = &args[2];

    let meta = std::fs::metadata(path)
        .map_err(|e| {
            anyhow::anyhow!(
                "stat failed: {}: {}",
                path,
                e
            )
        })?;

    let key = FileIdentity {
        dev: {
            let major =
                ((meta.st_dev() >> 8) & 0xfff) as u64;

            let minor =
                (
                    (meta.st_dev() & 0xff)
                    | ((meta.st_dev() >> 12) & 0xfffff00)
                ) as u64;

            ((major & 0xfff) << 20)
                | (minor & 0xfffff)
        },
        ino: meta.ino(),
    };
    
    let map_data = MapData::from_pin(
        "/sys/fs/bpf/airlock_policy"
    )?;

    let map =
        aya::maps::Map::HashMap(map_data);

    let mut map: HashMap<
        MapData,
        FileIdentity,
        PolicyEntry
    > = HashMap::try_from(
        map
    )?;

match action.as_str() {
        "--deny" => {
            let entry = PolicyEntry {
                action: ACTION_DENY,
                reserved: 0,
            };

            map.insert(key, entry, 0)?;

            println!(
                "denied: {} (dev={} ino={})",
                path,
                key.dev,
                key.ino
            );
        }

        "--allow" => {
            map.remove(&key)?;

            println!(
                "allowed: {} (dev={} ino={})",
                path,
                key.dev,
                key.ino
            );
        }

        _ => {
            eprintln!(
                "unknown action: {}",
                action
            );

            std::process::exit(1);
        }
    }

    Ok(())
}
