use std::fs;

use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::serialization::{DefaultGateSerializer, Write};

/// Fibonacci circuit, taken from plonky2 examples:
/// https://github.com/0xPolygonZero/plonky2/blob/v0.2.3/plonky2/examples/fibonacci.rs
/// Saves proof, public inputs and verification key to artifacts.
pub fn gen_factorial() {
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
    pw.set_target(initial_a, F::ZERO);
    pw.set_target(initial_b, F::ONE);

    let data = builder.build::<C>();

    let proof = data.prove(pw).unwrap();

    let mut proof_bytes = Vec::new();
    proof_bytes.write_proof(&proof.proof).unwrap();

    let mut pubs_bytes = Vec::new();
    pubs_bytes.write_usize(proof.public_inputs.len()).unwrap();
    pubs_bytes
        .write_field_vec(proof.public_inputs.as_slice())
        .unwrap();

    let vk_bytes = data
        .verifier_data()
        .to_bytes(&DefaultGateSerializer)
        .unwrap();

    fs::write("tests/artifacts/vk.bin", vk_bytes).unwrap();
    fs::write("tests/artifacts/proof.bin", proof_bytes).unwrap();
    fs::write("tests/artifacts/pubs.bin", pubs_bytes).unwrap();

    println!(
        "100th Fibonacci number mod |F| (starting with {}, {}) is: {}",
        proof.public_inputs[0], proof.public_inputs[1], proof.public_inputs[2]
    );

    data.verify(proof).unwrap()
}
