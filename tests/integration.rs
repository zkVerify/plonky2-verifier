#[path = "artifacts_generator.rs"]
mod artifacts_generator;

use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::serialization::DefaultGateSerializer;
use plonky2_verifier::{verify, DeserializeError, VerifyError};
use std::path::Path;

const PROOF_PATH: &str = "tests/artifacts/proof.bin";
const PUBS_PATH: &str = "tests/artifacts/pubs.bin";
const VK_PATH: &str = "tests/artifacts/vk.bin";

/// Proof, public inputs and verification key in serialized format.
type TestData = (Vec<u8>, Vec<u8>, Vec<u8>);

/// Ensures artifacts are generated and loads them.
/// Returns Result to handle potential loading errors.
fn load_data() -> Result<TestData, String> {
    if !Path::new(PROOF_PATH).exists()
        || !Path::new(PUBS_PATH).exists()
        || !Path::new(VK_PATH).exists()
    {
        println!("Generating artifacts...");
        artifacts_generator::gen_factorial();
    }

    let proof = std::fs::read(PROOF_PATH).map_err(|e| format!("Failed to load proof: {}", e))?;
    let pubs = std::fs::read(PUBS_PATH).map_err(|e| format!("Failed to load pubs: {}", e))?;
    let vk = std::fs::read(VK_PATH).map_err(|e| format!("Failed to load vk: {}", e))?;

    Ok((vk, proof, pubs))
}

/// Helper to run `verify` with preset types.
fn verify_non_generic(vk: &Vec<u8>, proof: &Vec<u8>, pubs: &Vec<u8>) -> Result<(), VerifyError> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    verify::<F, C, D>(
        vk.as_slice(),
        proof.as_slice(),
        pubs.as_slice(),
        &DefaultGateSerializer,
    )
}

#[test]
fn should_verify_valid_proof() {
    let (vk, proof, pubs) = load_data().expect("Failed to load data");
    assert!(verify_non_generic(&vk, &proof, &pubs).is_ok());
}

#[test]
fn should_not_deserialize_invalid_pubs() {
    let (vk, proof, mut pubs) = load_data().expect("Failed to load data");

    pubs[0] = pubs.first().unwrap().wrapping_add(1);

    assert!(
        matches!(
            verify_non_generic(&vk, &proof, &pubs),
            Err(VerifyError::InvalidData {
                cause: DeserializeError::InvalidProof
            })
        ),
        "Expected an InvalidProof error when `pubs` is corrupted"
    );
}

#[test]
fn should_not_verify_false_proof() {
    let (vk, mut proof, pubs) = load_data().expect("Failed to load data");

    let len = proof.len();
    proof[len - 1] = pubs.last().unwrap().wrapping_add(1);

    assert!(
        matches!(
            verify_non_generic(&vk, &proof, &pubs),
            Err(VerifyError::Failure { .. })
        ),
        "Expected a Failure error when `proof` is corrupted"
    );
}
