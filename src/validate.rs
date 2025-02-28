//! Validation crate centered for plonky2-verifier.

use crate::deserializer::deserialize_vk;
use crate::{DeserializeError, Plonky2Config, Vk};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CircuitConfig;
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
    /// Unsupported circuit config.
    #[snafu(display("Unsupported config"))]
    UnsupportedCircuitConfig,
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
        Plonky2Config::Keccak => {
            const D: usize = 2;
            type C = KeccakGoldilocksConfig;
            type F = <C as GenericConfig<D>>::F;

            validate_vk_inner::<F, C, D>(&vk.bytes)
        }
        Plonky2Config::Poseidon => {
            const D: usize = 2;
            type C = PoseidonGoldilocksConfig;
            type F = <C as GenericConfig<D>>::F;

            validate_vk_inner::<F, C, D>(&vk.bytes)
        }
    }
}

fn validate_vk_inner<F, C, const D: usize>(vk: &[u8]) -> ValidateResult
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    deserialize_vk::<F, C, D>(vk)
        .map_err(ValidateError::from)
        .and_then(|vk| {
            (vk.common.config == CircuitConfig::standard_recursion_config())
                .then_some(())
                .ok_or(ValidateError::UnsupportedCircuitConfig)
        })
}
