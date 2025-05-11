// rcli csv -i input.csv -o output.json --heder -d','
use std::fs;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, SubCommand, get_content, get_reader, process_csv, process_decode,
    process_encode, process_genpass, process_http_serve, process_text_key_generate,
    process_text_sign, process_text_verify,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;

            println!("{}", password);
            let estimate = zxcvbn::zxcvbn(&password, &[]);
            println!("Password strength: {:?}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;
                println!("{}", decode);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            rcli::TextSubCommand::Verify(opts) => {
                let reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(opts.sig)?;
                let verified = process_text_verify(reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("Signature verified");
                } else {
                    println!("Signature not verified");
                }
            }
            rcli::TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            rcli::HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }

    Ok(())
}
