// rcli csv -i input.csv -o output.json --heder -d','

use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
