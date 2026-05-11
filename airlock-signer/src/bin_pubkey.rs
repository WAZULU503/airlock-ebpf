use std::fs;

use anyhow::{
    anyhow,
    Result,
};

use ed25519_dalek::{
    SigningKey,
};

fn main() -> Result<()> {

    let secret_bytes =
        fs::read(
            "policy/master.key"
        )?;

    let signing_key =
        SigningKey::from_bytes(
            &secret_bytes
                .as_slice()
                .try_into()
                .map_err(|_| {
                    anyhow!(
                        "invalid 32-byte secret key"
                    )
                })?
        );

    let verifying_key =
        signing_key.verifying_key();

    fs::write(
        "policy/master.pub",
        verifying_key.to_bytes()
    )?;

    println!(
        "SUCCESS: master.pub generated"
    );

    Ok(())
}
