#![cfg(test)]
#[path = "artifacts_generator.rs"]
mod artifacts_generator;

use plonky2_verifier::{verify, DeserializeError, Proof, VerifyError, Vk};
use rstest::*;
use std::path::Path;

/// `TestData` for verification in serialized format.
struct TestData {
    vk: Vk,
    proof: Proof,
    proof_compressed: Proof,
    pubs: Vec<u8>,
}

#[fixture]
/// Ensures artifacts are generated and loads them.
fn valid_test_data() -> TestData {
    if !Path::new("tests/artifacts/vk.json").exists()
        || !Path::new("tests/artifacts/proof.json").exists()
        || !Path::new("tests/artifacts/proof_compressed.json").exists()
        || !Path::new("tests/artifacts/pubs.bin").exists()
    {
        println!("Generating artifacts...");
        artifacts_generator::gen_fibonacci();
    }

    let data =
        std::fs::read_to_string("tests/artifacts/vk.json").expect("Failed to read the vk.json");

    let vk: Vk = serde_json::from_str(&data).expect("Failed to deserialize JSON into Vk struct");

    let data = std::fs::read_to_string("tests/artifacts/proof.json")
        .expect("Failed to read the proof.json");

    let proof: Proof =
        serde_json::from_str(&data).expect("Failed to deserialize JSON into Proof struct");

    let data = std::fs::read_to_string("tests/artifacts/proof_compressed.json")
        .expect("Failed to read the proof_compressed.json");

    let proof_compressed: Proof =
        serde_json::from_str(&data).expect("Failed to deserialize JSON into Proof struct");

    let pubs = std::fs::read("tests/artifacts/pubs.bin").expect("Failed to read pubs.bin");

    TestData {
        vk,
        proof,
        proof_compressed,
        pubs,
    }
}

#[rstest]
fn should_verify_valid_proof(valid_test_data: TestData) {
    let TestData {
        vk, proof, pubs, ..
    } = valid_test_data;
    assert!(verify(&vk, &proof, &pubs).is_ok());
}

#[rstest]
fn should_verify_valid_compressed_proof(valid_test_data: TestData) {
    let TestData {
        vk,
        proof_compressed,
        pubs,
        ..
    } = valid_test_data;
    assert!(verify(&vk, &proof_compressed, &pubs).is_ok());
}

#[rstest]
fn should_not_deserialize_invalid_pubs(valid_test_data: TestData) {
    let TestData {
        vk,
        proof,
        mut pubs,
        ..
    } = valid_test_data;

    pubs[0] = pubs.first().unwrap().wrapping_add(1);

    assert!(
        matches!(
            verify(&vk, &proof, &pubs),
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
        ..
    } = valid_test_data;

    let len = proof.bytes.len();
    proof.bytes[len - 1] = pubs.last().unwrap().wrapping_add(1);

    assert!(
        matches!(verify(&vk, &proof, &pubs), Err(VerifyError::Failure { .. })),
        "Expected a Failure error when `proof` is corrupted"
    );
}
