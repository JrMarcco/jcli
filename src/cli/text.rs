use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::str::FromStr;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::{
    get_content, get_reader, process_key_gen, process_text_sign, process_text_verify, CmdExecutor,
};

use super::verify_path;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextOpt {
    #[command(name = "sign", about = "Sign a text with private key.")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify a signature with private key.")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate a random key.")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
    #[arg(long, default_value = ".", value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;

        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;

        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;

        if verified {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }

        Ok(())
    }
}

impl CmdExecutor for KeyGenerateOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let key_map = process_key_gen(self.format)?;

        for (k, v) in key_map {
            fs::write(self.output.join(k), v).await?;
        }

        Ok(())
    }
}
