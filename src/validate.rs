//! Validation crate centered for plonky2-verifier.

use crate::deserializer::deserialize_vk;
use crate::{DeserializeError, Plonky2Config, Vk};
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig, PoseidonGoldilocksConfig};
use snafu::Snafu;

/// Validation error.
#[derive(Debug, Snafu)]
pub enum ValidateError {
    /// Invalid data.
    #[snafu(display("Invalid data: [{}]", cause))]
    InvalidVK {
        /// Internal error.
        #[snafu(source)]
        cause: DeserializeError,
    },
}

impl From<DeserializeError> for ValidateError {
    fn from(value: DeserializeError) -> Self {
        ValidateError::InvalidVK { cause: value }
    }
}

/// Validation result.
type ValidateResult = Result<(), ValidateError>;

/// Validate `Vk`.
pub fn validate_vk(vk: &Vk) -> ValidateResult {
    match vk.config {
        Plonky2Config::Keccak => validate_vk_default_keccak(&vk.bytes),
        Plonky2Config::Poseidon => validate_vk_default_poseidon(&vk.bytes),
    }
}

/// Validate vk with preset Poseidon over Goldilocks config available in `plonky2`.
pub fn validate_vk_default_poseidon(vk: &[u8]) -> ValidateResult {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    deserialize_vk::<F, C, D>(vk)
        .map(|_| ())
        .map_err(ValidateError::from)
}

/// Validate vk with preset Keccak over Goldilocks config available in `plonky2`.
pub fn validate_vk_default_keccak(vk: &[u8]) -> ValidateResult {
    const D: usize = 2;
    type C = KeccakGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    deserialize_vk::<F, C, D>(vk)
        .map(|_| ())
        .map_err(ValidateError::from)
}
