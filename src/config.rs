//! Configuration for `plonky2` verifier.
use serde::{Deserialize, Serialize};

#[cfg(feature = "converter")]
use clap::ValueEnum;

/// Config for `Plonky2` proving system with options, acceptable by `zkVerify`.
#[derive(Copy, Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "converter", derive(ValueEnum))]
pub enum Plonky2Config {
    /// Preset Keccak over Goldilocks config available in `plonky2`
    Keccak,
    /// Preset Poseidon over Goldilocks config available in `plonky2`
    #[default]
    Poseidon,
}
