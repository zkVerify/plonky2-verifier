#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod deserializer;
pub mod validate;

use deserializer::{deserialize_proof_with_pubs, deserialize_vk};

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig, PoseidonGoldilocksConfig};
use snafu::Snafu;

pub use deserializer::{custom::ZKVerifyGateSerializer, DeserializeError};

/// Verification error.
#[derive(Debug, Snafu)]
pub enum VerifyError {
    /// Invalid data.
    #[snafu(display("Invalid data for verification: [{}]", cause))]
    InvalidData {
        /// Internal error.
        #[snafu(source)]
        cause: DeserializeError,
    },
    /// Failure.
    #[snafu(display("Failed to verify"))]
    Failure,
}

impl From<DeserializeError> for VerifyError {
    fn from(value: DeserializeError) -> Self {
        VerifyError::InvalidData { cause: value }
    }
}

/// Verify the given proof `proof` and public inputs `pubs` using verification key `vk`.
/// Use the given verification key `vk` to verify the proof `proof` against the public inputs `pubs`.
pub fn verify<F, C, const D: usize>(vk: &[u8], proof: &[u8], pubs: &[u8]) -> Result<(), VerifyError>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    let vk = deserialize_vk::<F, C, D>(vk)?;
    let proof = deserialize_proof_with_pubs::<F, C, D>(proof, pubs, &vk.common)?;

    vk.verify(proof).map_err(|_| VerifyError::Failure)
}

/// Verification with preset Poseidon over Goldilocks config available in `plonky2`.
pub fn verify_default_poseidon(vk: &[u8], proof: &[u8], pubs: &[u8]) -> Result<(), VerifyError> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    verify::<F, C, D>(vk, proof, pubs)
}

/// Verification with preset Keccak over Goldilocks config available in `plonky2`.
pub fn verify_default_keccak(vk: &[u8], proof: &[u8], pubs: &[u8]) -> Result<(), VerifyError> {
    const D: usize = 2;
    type C = KeccakGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    verify::<F, C, D>(vk, proof, pubs)
}
