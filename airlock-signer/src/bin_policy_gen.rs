use std::{
    fs,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use anyhow::Result;

use postcard::to_allocvec;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
struct PolicyEntry {
    target_path: String,
    action: u32,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
struct PolicyBody {
    version: u32,

    issued_at_unix: i64,

    expires_at_unix: Option<i64>,

    entries: Vec<PolicyEntry>,
}

fn main() -> Result<()> {

    let now =
        SystemTime::now()
            .duration_since(
                UNIX_EPOCH
            )?
            .as_secs() as i64;

    let body = PolicyBody {
        version: 1,

        issued_at_unix: now,

        expires_at_unix: Some(
            now + 86400
        ),

        entries: vec![
            PolicyEntry {
                target_path:
                    "/usr/bin/ping"
                        .into(),

                action: 2,
            },

            PolicyEntry {
                target_path:
                    "/usr/bin/dash"
                        .into(),

                action: 1,
            },
        ],
    };

    let bytes =
        to_allocvec(&body)?;

    fs::write(
        "policy/policy.bin",
        bytes
    )?;

    println!(
        "SUCCESS: policy.bin generated"
    );

    Ok(())
}
