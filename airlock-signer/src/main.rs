use std::fs;

use anyhow::{
    anyhow,
    Result,
};

use ed25519_dalek::{
    Signer,
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

    let body_bytes =
        fs::read(
            "policy/policy.bin"
        )?;

    let signature =
        signing_key.sign(
            &body_bytes
        );

    fs::write(
        "policy/policy.sig",
        signature.to_bytes()
    )?;

    println!(
        "SUCCESS: policy.sig generated"
    );

    Ok(())
}
