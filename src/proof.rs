//! Proof for `plonky2` in a format, acceptable by `zkVerify`.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[cfg(feature = "std")]
extern crate std;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

/// `Proof` encapsulating compression parameter.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Proof {
    /// Is `proof` compressed.
    pub compressed: bool,
    /// Serialized `Proof` or `CompressedProof` from `plonky2`.
    #[serde_as(as = "serde_with::hex::Hex")]
    pub bytes: Vec<u8>,
}

#[cfg(feature = "converter")]
impl Proof {
    /// Serializes the entire `Proof` struct to a binary format.
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serialization to bytes failed")
    }

    /// Serializes the entire `Proof` struct to a hex-encoded string.
    pub fn as_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }
}
