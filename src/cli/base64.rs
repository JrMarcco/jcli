use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Opt {
    #[command(name = "encode", about = "Encode a string to base64.")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string.")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format.")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url_safe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for Base64EncodeOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let encoded = crate::process_encode(&self.input, self.format)?;
        println!("{}", encoded);

        Ok(())
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let decoded = crate::process_decode(&self.input, self.format)?;
        println!("{}", decoded);

        Ok(())
    }
}
