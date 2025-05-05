use std::{fs::File, io::Read};

use crate::Base64Format;
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let content = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
    };

    println!("{}", content);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let content = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
    };

    // TODO: decode data migght not be string (but for this example, we assume it is)
    let decoded = String::from_utf8(content)?;
    println!("{}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    // 如何处理作用域中不同的反回类型
    // 类型擦除
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        process_decode(input, format).unwrap();
    }
}
