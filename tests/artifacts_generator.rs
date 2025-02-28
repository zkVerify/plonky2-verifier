use std::fs;

use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::serialization::Write;
use plonky2_verifier::{Plonky2Config, Proof, Vk, ZKVerifyGateSerializer};

/// Fibonacci circuit, taken from plonky2 examples:
/// https://github.com/0xPolygonZero/plonky2/blob/v0.2.3/plonky2/examples/fibonacci.rs
/// Saves proof, public inputs and verification key to artifacts.
pub fn gen_fibonacci() {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let initial_a = builder.add_virtual_target();
    let initial_b = builder.add_virtual_target();
    let mut prev_target = initial_a;
    let mut cur_target = initial_b;
    for _ in 0..99 {
        let temp = builder.add(prev_target, cur_target);
        prev_target = cur_target;
        cur_target = temp;
    }

    // Public inputs are the two initial values (provided below) and the result (which is valid).
    builder.register_public_input(initial_a);
    builder.register_public_input(initial_b);
    builder.register_public_input(cur_target);

    // Provide initial values.
    let mut pw = PartialWitness::new();
    pw.set_target(initial_a, F::ZERO).unwrap();
    pw.set_target(initial_b, F::ONE).unwrap();

    let data = builder.build::<C>();

    let proof = data.prove(pw).unwrap();

    let mut proof_bytes = Vec::new();
    proof_bytes.write_proof(&proof.proof).unwrap();

    let compressed_proof = proof
        .clone()
        .compress(&data.verifier_only.circuit_digest, &data.common)
        .unwrap();
    let mut compressed_proof_bytes = Vec::new();
    compressed_proof_bytes
        .write_compressed_proof(&compressed_proof.proof)
        .unwrap();

    let mut pubs_bytes = Vec::new();
    pubs_bytes.write_usize(proof.public_inputs.len()).unwrap();
    pubs_bytes
        .write_field_vec(proof.public_inputs.as_slice())
        .unwrap();

    let vk_bytes = data
        .verifier_data()
        .to_bytes(&ZKVerifyGateSerializer)
        .unwrap();

    let vk = Vk {
        config: Plonky2Config::Poseidon,
        bytes: vk_bytes,
    };

    let proof = Proof {
        compressed: false,
        bytes: proof_bytes,
    };

    let proof_compressed = Proof {
        compressed: true,
        bytes: compressed_proof_bytes,
    };

    serde_json::to_writer(&fs::File::create("tests/artifacts/vk.json").unwrap(), &vk).unwrap();
    serde_json::to_writer(
        &fs::File::create("tests/artifacts/proof.json").unwrap(),
        &proof,
    )
    .unwrap();
    serde_json::to_writer(
        &fs::File::create("tests/artifacts/proof_compressed.json").unwrap(),
        &proof_compressed,
    )
    .unwrap();
    fs::write("tests/artifacts/pubs.bin", pubs_bytes).unwrap();
}
