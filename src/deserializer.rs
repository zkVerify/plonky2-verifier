use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::{CommonCircuitData, VerifierCircuitData};
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::GateSerializer;

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

pub(crate) fn deserialize_vk<F, C, const D: usize>(
    vk: &[u8],
    gs: &dyn GateSerializer<F, D>,
) -> Result<VerifierCircuitData<F, C, D>, DeserializeError>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    VerifierCircuitData::<F, C, D>::from_bytes(Vec::from(vk), gs)
        .map_err(|_| DeserializeError::InvalidVerificationKey)
}

pub(crate) fn deserialize_proof_with_pubs<F, C, const D: usize>(
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
