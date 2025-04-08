use std::fs::File;

use anyhow::Result;
use clap::Parser;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::Write;
use plonky2_verifier::ZKVerifyGateSerializer;

/// Simple program for generating proof, vk, and pubs binary files for
/// an instance with configurable number of cycles.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Exponent in the power of 2 specifying the number of cycles.
    #[arg(short, long)]
    power: u32,
}

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

/// An example of using Plonky2 to prove a statement of the form
/// "I know the 100th element of the Fibonacci sequence, starting with constants a and b."
/// When a == 0 and b == 1, this is proving knowledge of the 100th (standard) Fibonacci number.
/// This example also serializes the circuit data and proof to JSON files.
fn main() -> Result<()> {
    let args = Args::parse();

    let num_cycles: u64 = 1 << args.power;

    let (data, proof) = build_cicruit_and_proof(num_cycles);

    println!("degree_bits = {}", data.common.fri_params.degree_bits);

    let _ = data.verify(proof.clone());

    let vk_bytes = data
        .verifier_data()
        .to_bytes(&ZKVerifyGateSerializer)
        .unwrap();
    save_to_bin_file(&vk_bytes, "vk.bin").unwrap();

    let mut proof_bytes = Vec::new();
    proof_bytes.write_proof(&proof.proof).unwrap();
    save_to_bin_file(&proof_bytes, "proof.bin").unwrap();

    let mut pubs_bytes = Vec::new();
    pubs_bytes.write_usize(proof.public_inputs.len()).unwrap();
    pubs_bytes
        .write_field_vec(proof.public_inputs.as_slice())
        .unwrap();
    save_to_bin_file(&pubs_bytes, "pubs.bin").unwrap();

    Ok(())
}

fn build_cicruit_and_proof(
    num_cycles: u64,
) -> (
    CircuitData<GoldilocksField, PoseidonGoldilocksConfig, D>,
    ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, D>,
) {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let initial_a = builder.add_virtual_target();
    let initial_b = builder.add_virtual_target();
    let mut prev_target = initial_a;
    let mut cur_target = initial_b;
    for _ in 0..num_cycles {
        let temp = builder.add(prev_target, cur_target);
        prev_target = cur_target;
        cur_target = temp;
    }

    // Public inputs are the two initial values (provided below) and the result (which is generated).
    builder.register_public_input(initial_a);
    builder.register_public_input(initial_b);
    builder.register_public_input(cur_target);

    // Provide initial values.
    let mut pw = PartialWitness::new();
    let _ = pw.set_target(initial_a, F::ZERO);
    let _ = pw.set_target(initial_b, F::ONE);

    let data = builder.build::<C>();
    let proof = data.prove(pw).unwrap();

    (data, proof)
}

fn save_to_bin_file(data: &Vec<u8>, filename: &str) -> Result<()> {
    use std::io::Write;
    let mut file = File::create(filename)?; // Create or overwrite the file
    file.write_all(data)?; // Write all bytes
    Ok(())
}
