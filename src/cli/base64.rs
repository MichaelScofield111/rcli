use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_input_file;

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    UrlSafe,
    Standard,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl From<Base64Format> for &'static str {
    fn from(f: Base64Format) -> Self {
        match f {
            Base64Format::UrlSafe => "urlsafe",
            Base64Format::Standard => "standard",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "urlsafe" => Ok(Base64Format::UrlSafe),
            "standard" => Ok(Base64Format::Standard),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::UrlSafe => write!(f, "urlsafe"),
            Base64Format::Standard => write!(f, "standard"),
        }
    }
}
