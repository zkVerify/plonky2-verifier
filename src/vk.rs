//! Verification key for `plonky2` in a format, acceptable by `zkVerify`.

use crate::config::Plonky2Config;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[cfg(feature = "std")]
extern crate std;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

/// `Vk` encapsulating configuration.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Vk {
    /// Configuration for this `Vk`.
    pub config: Plonky2Config,
    /// Serialized `VerifierCircuitData` from `plonky2`.
    #[serde_as(as = "serde_with::hex::Hex")]
    pub bytes: Vec<u8>,
}

#[cfg(feature = "converter")]
impl Vk {
    /// Serializes the entire `Vk` struct to a binary format.
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serialization to bytes failed")
    }

    /// Serializes the entire `Vk` struct to a hex-encoded string.
    pub fn as_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }
}
