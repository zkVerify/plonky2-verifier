use anyhow::{Context, Result};
use clap::ValueEnum;
use plonky2_verifier::Plonky2Config;
use plonky2_verifier::Vk;
use std::io;

/// Supported formats for input verifier circuit data.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum VerifierCircuitDataFormat {
    /// Raw binary bytes.
    #[default]
    Bytes,
    /// Hex-encoded string (with optional "0x" prefix).
    Hex,
}

impl VerifierCircuitDataFormat {
    /// Decodes the verifier circuit data from the specified format.
    pub fn decode(&self, vk_bytes: Vec<u8>, config: Plonky2Config) -> Result<Vk> {
        let vk_bytes = match self {
            VerifierCircuitDataFormat::Bytes => vk_bytes,
            VerifierCircuitDataFormat::Hex => {
                let hex_str = vk_bytes.strip_prefix(b"0x").unwrap_or(&vk_bytes);
                hex::decode(hex_str).with_context(|| {
                    format!(
                        "Failed to decode hex string: {:?}",
                        String::from_utf8_lossy(hex_str)
                    )
                })?
            }
        };

        Ok(Vk {
            vk_bytes: bytes,
            config,
        })
    }
}

/// Formats for outputting serialized verifier circuit data.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum Format {
    /// JSON format (pretty-printed).
    #[default]
    Json,
    /// Raw binary bytes.
    Bytes,
    /// Hex-encoded string.
    Hex,
}

impl Format {
    /// Writes the verifier circuit data (`Vk`) to the specified output in the selected format.
    pub fn write_vk(&self, vk: &Vk, out: &mut dyn io::Write) -> Result<()> {
        match self {
            Format::Json => {
                serde_json::to_writer_pretty(out, vk).context("Failed to serialize Vk as JSON")?;
            }
            Format::Bytes => {
                out.write_all(&vk.as_bytes())
                    .context("Failed to write Vk as raw bytes")?;
            }
            Format::Hex => {
                out.write_all(vk.as_hex().as_bytes())
                    .context("Failed to write Vk as a hex string")?;
            }
        }
        Ok(())
    }
}
