use anyhow::{Context, Result};
use clap::ValueEnum;
use plonky2_verifier::Vk;
use plonky2_verifier::{Plonky2Config, Proof};
use std::io;

/// Supported formats for input file.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum InFormat {
    /// Raw binary bytes.
    #[default]
    Bytes,
    /// Hex-encoded string (with optional "0x" prefix).
    Hex,
}

impl InFormat {
    /// Decodes the verifier circuit data from the specified format.
    pub fn decode_vk(&self, vk_bytes: Vec<u8>, config: Plonky2Config) -> Result<Vk> {
        let bytes = match self {
            InFormat::Bytes => vk_bytes,
            InFormat::Hex => {
                let hex_str = vk_bytes.strip_prefix(b"0x").unwrap_or(&vk_bytes);
                hex::decode(hex_str).with_context(|| {
                    format!(
                        "Failed to decode hex string: {:?}",
                        String::from_utf8_lossy(hex_str)
                    )
                })?
            }
        };

        Ok(Vk { bytes, config })
    }

    /// Decodes the proof from the specified format.
    pub fn decode_proof(&self, proof_bytes: Vec<u8>, compressed: bool) -> Result<Proof> {
        let bytes = match self {
            InFormat::Bytes => proof_bytes,
            InFormat::Hex => {
                let hex_str = proof_bytes.strip_prefix(b"0x").unwrap_or(&proof_bytes);
                hex::decode(hex_str).with_context(|| {
                    format!(
                        "Failed to decode hex string: {:?}",
                        String::from_utf8_lossy(hex_str)
                    )
                })?
            }
        };

        Ok(Proof { bytes, compressed })
    }
}

/// Formats for outputting serialized verifier circuit data.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum OutFormat {
    /// JSON format (pretty-printed).
    #[default]
    Json,
    /// Raw binary bytes.
    Bytes,
    /// Hex-encoded string.
    Hex,
}

impl OutFormat {
    /// Writes the verifier circuit data (`Vk`) to the specified output in the selected format.
    pub fn write_vk(&self, vk: &Vk, out: &mut dyn io::Write) -> Result<()> {
        match self {
            OutFormat::Json => {
                serde_json::to_writer_pretty(out, vk).context("Failed to serialize Vk as JSON")?;
            }
            OutFormat::Bytes => {
                out.write_all(&vk.as_bytes())
                    .context("Failed to write Vk as raw bytes")?;
            }
            OutFormat::Hex => {
                out.write_all(vk.as_hex().as_bytes())
                    .context("Failed to write Vk as a hex string")?;
            }
        }
        Ok(())
    }

    /// Writes the proof (`Proof`) to the specified output in the selected format.
    pub fn write_proof(&self, proof: &Proof, out: &mut dyn io::Write) -> Result<()> {
        match self {
            OutFormat::Json => {
                serde_json::to_writer_pretty(out, proof)
                    .context("Failed to serialize Proof as JSON")?;
            }
            OutFormat::Bytes => {
                out.write_all(&proof.as_bytes())
                    .context("Failed to write Proof as raw bytes")?;
            }
            OutFormat::Hex => {
                out.write_all(proof.as_hex().as_bytes())
                    .context("Failed to write Proof as a hex string")?;
            }
        }
        Ok(())
    }
}
