#![cfg(test)]
#[path = "artifacts_generator.rs"]
mod artifacts_generator;

use plonky2_verifier::{verify_default_poseidon, DeserializeError, VerifyError};
use rstest::*;
use std::path::Path;

/// `TestData` for verification in serialized format.
struct TestData {
    vk: Vec<u8>,
    proof: Vec<u8>,
    pubs: Vec<u8>,
}

#[fixture]
/// Ensures artifacts are generated and loads them.
fn valid_test_data() -> TestData {
    if !Path::new("tests/artifacts/vk.bin").exists()
        || !Path::new("tests/artifacts/proof.bin").exists()
        || !Path::new("tests/artifacts/pubs.bin").exists()
    {
        println!("Generating artifacts...");
        artifacts_generator::gen_factorial();
    }

    TestData {
        vk: include_bytes!("artifacts/vk.bin").to_vec(),
        proof: include_bytes!("artifacts/proof.bin").to_vec(),
        pubs: include_bytes!("artifacts/pubs.bin").to_vec(),
    }
}

#[rstest]
fn should_verify_valid_proof(valid_test_data: TestData) {
    let TestData { vk, proof, pubs } = valid_test_data;
    assert!(verify_default_poseidon(&vk, &proof, &pubs).is_ok());
}

#[rstest]
fn should_not_deserialize_invalid_pubs(valid_test_data: TestData) {
    let TestData {
        vk,
        proof,
        mut pubs,
    } = valid_test_data;

    pubs[0] = pubs.first().unwrap().wrapping_add(1);

    assert!(
        matches!(
            verify_default_poseidon(&vk, &proof, &pubs),
            Err(VerifyError::InvalidData {
                cause: DeserializeError::InvalidProof
            })
        ),
        "Expected an InvalidProof error when `pubs` is corrupted"
    );
}

#[rstest]
fn should_not_verify_false_proof(valid_test_data: TestData) {
    let TestData {
        vk,
        mut proof,
        pubs,
    } = valid_test_data;

    let len = proof.len();
    proof[len - 1] = pubs.last().unwrap().wrapping_add(1);

    assert!(
        matches!(
            verify_default_poseidon(&vk, &proof, &pubs),
            Err(VerifyError::Failure { .. })
        ),
        "Expected a Failure error when `proof` is corrupted"
    );
}
