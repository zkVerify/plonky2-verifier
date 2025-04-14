use std::fs::File;
use std::str::FromStr;
use std::usize;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{GenericConfig, KeccakGoldilocksConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::Write;
use plonky2_verifier::ZKVerifyGateSerializer;

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, ValueEnum)]
enum HashFunction {
    Keccak,
    #[default]
    Poseidon,
}

impl FromStr for HashFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "keccak" => Ok(HashFunction::Keccak),
            "poseidon" => Ok(HashFunction::Poseidon),
            _ => Err(format!("Invalid hash function family: {}", s)),
        }
    }
}

/// Simple program for generating proof, vk, and pubs binary files for
/// an instance with configurable number of cycles.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Exponent in the power of 2 specifying the number of cycles.
    #[arg(short, long)]
    power: u32,
    /// Hash function family to be used in proof generation.
    #[arg(long, value_enum, default_value_t = HashFunction::Poseidon)]
    hash: HashFunction,
    /// Use compression.
    #[arg(short, long, default_value_t = false)]
    compress: bool,
}

/// An example of using Plonky2 to prove a statement of the form
/// "I know the 100th element of the Fibonacci sequence, starting with constants a and b."
/// When a == 0 and b == 1, this is proving knowledge of the 100th (standard) Fibonacci number.
/// This example also serializes the circuit data and proof to JSON files.
fn main() -> Result<()> {
    let args = Args::parse();

    let num_cycles: u64 = 1 << args.power;

    match args.hash {
        HashFunction::Poseidon => {
            const D: usize = 2;
            type C = PoseidonGoldilocksConfig;
            type F = <C as GenericConfig<D>>::F;
            main_impl::<D, C, F>(num_cycles, args.compress)
        }
        HashFunction::Keccak => {
            const D: usize = 2;
            type C = KeccakGoldilocksConfig;
            type F = <C as GenericConfig<D>>::F;
            main_impl::<D, C, F>(num_cycles, args.compress)
        }
    }
}

fn main_impl<const D: usize, C, F>(num_cycles: u64, compress: bool) -> Result<()>
where
    C: GenericConfig<D, F = F>,
    F: RichField + Extendable<D>,
{
    let (data, proof) = build_cicruit_and_proof::<D, C, F>(num_cycles);

    println!("degree_bits = {}", data.common.fri_params.degree_bits);
    println!("Compress: {}", compress);

    let vk_bytes = data
        .verifier_data()
        .to_bytes(&ZKVerifyGateSerializer)
        .unwrap();

    let mut proof_bytes = Vec::new();
    let mut pubs_bytes = Vec::new();
    if compress {
        let compressed_proof = data.compress(proof.clone())?;
        proof_bytes
            .write_compressed_proof(&compressed_proof.proof)
            .unwrap();
        pubs_bytes
            .write_usize(compressed_proof.public_inputs.len())
            .unwrap();
        pubs_bytes
            .write_field_vec(compressed_proof.public_inputs.as_slice())
            .unwrap();
    } else {
        proof_bytes.write_proof(&proof.proof).unwrap();
        pubs_bytes.write_usize(proof.public_inputs.len()).unwrap();
        pubs_bytes
            .write_field_vec(proof.public_inputs.as_slice())
            .unwrap();
    }

    data.verify(proof.clone()).unwrap();

    save_to_bin_file(&vk_bytes, "vk.bin").unwrap();
    save_to_bin_file(&proof_bytes, "proof.bin").unwrap();
    save_to_bin_file(&pubs_bytes, "pubs.bin").unwrap();

    Ok(())
}

fn build_cicruit_and_proof<const D: usize, C, F>(
    num_cycles: u64,
) -> (CircuitData<F, C, D>, ProofWithPublicInputs<F, C, D>)
where
    C: GenericConfig<D, F = F>,
    F: RichField + Extendable<D>,
{
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
