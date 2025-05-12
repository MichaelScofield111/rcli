use clap::Parser;

use crate::CmdExecutor;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = false)]
    pub uppercase: bool,
    #[arg(long, default_value_t = false)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", ret);

        let estimate = zxcvbn::zxcvbn(&ret, &[]);

        eprintln!("Passworld strength: {}", estimate.score());
        Ok(())
    }
}
