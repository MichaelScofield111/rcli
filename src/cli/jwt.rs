use anyhow::Ok;
use clap::Parser;
use jsonwebtoken::Header;
use serde::{Deserialize, Serialize};

use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign a JWT")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "Verify a JWT")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser, Deserialize, Serialize)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub company: String,
    #[arg(long)]
    pub exp: usize,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(long)]
    pub input: String,
}

impl CmdExecutor for JwtSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            JwtSubCommand::Sign(opts) => opts.execute().await,
            JwtSubCommand::Verify(opts) => opts.execute().await,
        }
    }
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode_data = crate::jwt_sign_by_hs256(&Header::default(), &self, "secret")?;

        println!("{:?}", encode_data);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode_data = crate::jwt_verify_by_hs256(&self.input)?;

        println!("{:?}", decode_data.claims);
        println!("{:?}", decode_data.header);
        Ok(())
    }
}
