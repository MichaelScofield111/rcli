mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::base64::Base64Format;
pub use self::csv::OutputFormat;
pub use base64::Base64SubCommand;
use clap::Parser;
use csv::CsvOpts;
use genpass::GenPassOpts;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exist
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File not found"));
        assert_eq!(verify_input_file("cargo.toml"), Ok("cargo.toml".into()));
    }
}
