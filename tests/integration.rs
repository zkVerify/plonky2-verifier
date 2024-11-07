#[path = "artifacts_generator.rs"]
mod artifacts_generator;

use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::serialization::DefaultGateSerializer;
use plonky2_verifier::{verify, DeserializeError, VerifyError};

fn load_data() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    // Ensure artifacts exist by running `gen_factorial` only if needed
    if !std::path::Path::new("tests/artifacts/proof.bin").exists() ||
        !std::path::Path::new("tests/artifacts/pubs.bin").exists() ||
        !std::path::Path::new("tests/artifacts/vk.bin").exists()
    {
        println!("Generating artifacts...");
        artifacts_generator::gen_factorial();
    }
    let proof = std::fs::read("tests/artifacts/proof.bin").unwrap();
    let pubs = std::fs::read("tests/artifacts/pubs.bin").unwrap();
    let vk = std::fs::read("tests/artifacts/vk.bin").unwrap();

    (vk, proof, pubs)
}

fn verify_non_generic(vk: &Vec<u8>, proof: &Vec<u8>, pubs: &Vec<u8>) -> Result<(), VerifyError> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    verify::<F, C, D>(vk.as_slice(), proof.as_slice(), pubs.as_slice(), &DefaultGateSerializer)
}

#[test]
fn should_verify_valid_proof() {
    let (vk, proof, pubs) = load_data();

    assert!(verify_non_generic(&vk, &proof, &pubs).is_ok());
}

#[test]
fn should_not_deserialize_invalid_pubs() {
    let (vk, proof, mut pubs) = load_data();

    pubs[0] = pubs.first().unwrap().wrapping_add(1);

    assert!(matches!(
        verify_non_generic(&vk, &proof, &pubs),
        Err(VerifyError::InvalidData {
            cause: DeserializeError::InvalidProof
        })
    ));
}

#[test]
fn should_not_verify_false_proof() {
    let (vk, proof, mut pubs) = load_data();
    
    let len = pubs.len();
    pubs[len - 1] = pubs.last().unwrap().wrapping_add(1);

    assert!(matches!(
        verify_non_generic(&vk, &proof, &pubs),
        Err(VerifyError::Failure { .. })
    ));
}