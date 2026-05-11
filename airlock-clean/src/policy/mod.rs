use std::{
    error::Error,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use ed25519_dalek::{
    Signature,
    Verifier,
    VerifyingKey,
};

use serde::{
    Deserialize,
    Serialize,
};

use sha2::{
    Digest,
    Sha256,
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[serde(deny_unknown_fields)]
pub struct PolicyEntry {
    pub target_path: String,
    pub action: u32,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
#[serde(deny_unknown_fields)]
pub struct PolicyBody {
    pub version: u32,

    pub issued_at_unix: i64,

    pub expires_at_unix: Option<i64>,

    pub entries: Vec<PolicyEntry>,
}

pub struct SignedPolicy {
    pub policy_id: [u8; 8],

    pub body: PolicyBody,

    pub signature: Signature,
}

impl SignedPolicy {
    pub fn verify_and_decode(
        body_bytes: &[u8],
        sig_bytes: &[u8],
        pub_key_bytes: &[u8],
    ) -> Result<Self, Box<dyn Error>> {

        let verifying_key =
            VerifyingKey::from_bytes(
                pub_key_bytes
                    .try_into()
                    .map_err(|_| {
                        "invalid 32-byte public key"
                    })?
            )?;

        let signature =
            Signature::from_bytes(
                sig_bytes
                    .try_into()
                    .map_err(|_| {
                        "invalid 64-byte signature"
                    })?
            );

        verifying_key
            .verify_strict(
                body_bytes,
                &signature,
            )
            .map_err(|_| {
                "signature verification failed"
            })?;

        let body: PolicyBody =
            postcard::from_bytes(
                body_bytes
            )?;

        let mut hasher =
            Sha256::new();

        hasher.update(
            body_bytes
        );

        let digest =
            hasher.finalize();

        let mut policy_id =
            [0u8; 8];

        policy_id.copy_from_slice(
            &digest[..8]
        );

        let now =
            SystemTime::now()
                .duration_since(
                    UNIX_EPOCH
                )?
                .as_secs() as i64;

        if let Some(expiry) =
            body.expires_at_unix
        {
            if now > expiry {
                return Err(
                    "policy expired".into()
                );
            }
        }

        Ok(
            Self {
                policy_id,
                body,
                signature,
            }
        )
    }
}
