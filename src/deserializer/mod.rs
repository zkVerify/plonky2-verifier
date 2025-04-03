pub mod custom;

use custom::ZKVerifyGateSerializer;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::{CommonCircuitData, VerifierCircuitData};
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::{CompressedProofWithPublicInputs, ProofWithPublicInputs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use snafu::Snafu;

/// Deserialization error.
#[derive(Debug, Snafu)]
pub enum DeserializeError {
    /// Invalid proof or public inputs.
    #[snafu(display("Invalid proof or public inputs for deserialization"))]
    InvalidProof,
    /// Invalid verification key.
    #[snafu(display("Invalid verification key for deserialization"))]
    InvalidVerificationKey,
}

/// Deserialize a `Vk` from bytes to `VerifierCircuitData`.
pub fn deserialize_vk<F, C, const D: usize>(
    vk: &[u8],
) -> Result<VerifierCircuitData<F, C, D>, DeserializeError>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    VerifierCircuitData::<F, C, D>::from_bytes(Vec::from(vk), &ZKVerifyGateSerializer)
        .map_err(|_| DeserializeError::InvalidVerificationKey)
}

/// Combine a `Proof` and `Pubs` and deserialize into `ProofWithPublicInputs`.
pub fn deserialize_proof_with_pubs<F, C, const D: usize>(
    proof: &[u8],
    pubs: &[u8],
    common_data: &CommonCircuitData<F, D>,
) -> Result<ProofWithPublicInputs<F, C, D>, DeserializeError>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    let mut bytes = Vec::with_capacity(proof.len() + pubs.len());
    bytes.extend_from_slice(proof);
    bytes.extend_from_slice(pubs);

    ProofWithPublicInputs::<F, C, D>::from_bytes(bytes, common_data)
        .map_err(|_| DeserializeError::InvalidProof)
}

/// Combine a compressed `Proof` and `Pubs` and deserialize into `CompressedProofWithPublicInputs`.
pub fn deserialize_compressed_proof_with_pubs<F, C, const D: usize>(
    proof: &[u8],
    pubs: &[u8],
    common_data: &CommonCircuitData<F, D>,
) -> Result<CompressedProofWithPublicInputs<F, C, D>, DeserializeError>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    let offset = size_of::<usize>();
    if pubs.len() < offset {
        return Err(DeserializeError::InvalidProof);
    }
    let mut bytes = Vec::with_capacity(proof.len() + pubs.len());
    bytes.extend_from_slice(proof);
    bytes.extend_from_slice(&pubs[offset..]);

    CompressedProofWithPublicInputs::<F, C, D>::from_bytes(bytes, common_data)
        .map_err(|_| DeserializeError::InvalidProof)
}
