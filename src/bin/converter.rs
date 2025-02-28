mod formats;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use plonky2_verifier::Plonky2Config;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "plonky2-converter",
    about = "Converts plonky2 formats",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Serialize VerifierCircuitData into zkVerify format.
    Vk(VkArgs),
    /// Serialize Proof or CompressedProof into zkVerify format.
    Proof(ProofArgs),
}

#[derive(Debug, Parser)]
struct VkArgs {
    #[arg(short, long, value_enum, default_value_t = formats::InFormat::default())]
    in_fmt: formats::InFormat,

    #[arg(short, long, value_enum, default_value_t = formats::OutFormat::default())]
    out_fmt: formats::OutFormat,

    input: PathBuf,
    output: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value_t = Plonky2Config::default())]
    config: Plonky2Config,
}

#[derive(Debug, Parser)]
struct ProofArgs {
    #[arg(short, long, value_enum, default_value_t = formats::InFormat::default())]
    in_fmt: formats::InFormat,

    #[arg(short, long, value_enum, default_value_t = formats::OutFormat::default())]
    out_fmt: formats::OutFormat,

    input: PathBuf,
    output: Option<PathBuf>,

    #[arg(short, long, value_enum)]
    compressed: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Vk(args) => handle_vk(args),
        Commands::Proof(args) => handle_proof(args),
    }
}

fn handle_vk(args: VkArgs) -> Result<()> {
    log::info!("Processing VK command with args: {:?}", args);

    let vk_bytes = std::fs::read(&args.input).with_context(|| {
        format!(
            "Could not read file {:?}. Ensure the file exists and is accessible.",
            &args.input
        )
    })?;
    let vk = args.in_fmt.decode_vk(vk_bytes, args.config)?;
    let mut out = out_file(args.output.as_ref())?;
    args.out_fmt.write_vk(&vk, &mut out)?;

    log::info!("Successfully wrote output");
    Ok(())
}

fn handle_proof(args: ProofArgs) -> Result<()> {
    log::info!("Processing Proof command with args: {:?}", args);

    let proof_bytes = std::fs::read(&args.input).with_context(|| {
        format!(
            "Could not read file {:?}. Ensure the file exists and is accessible.",
            &args.input
        )
    })?;
    let proof = args.in_fmt.decode_proof(proof_bytes, args.compressed)?;
    let mut out = out_file(args.output.as_ref())?;
    args.out_fmt.write_proof(&proof, &mut out)?;

    log::info!("Successfully wrote output");
    Ok(())
}

fn out_file(output: Option<&PathBuf>) -> Result<Box<dyn Write>> {
    match output {
        Some(path) => {
            Ok(Box::new(File::create(path).with_context(|| {
                format!("Failed to create output file {:?}", path)
            })?))
        }
        None => Ok(Box::new(io::stdout())),
    }
}
