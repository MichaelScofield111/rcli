use clap::Parser;
// rcli csv -i input.csv -o output.json --heder -d','
use rcli::process_csv;
use rcli::{Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }

    Ok(())
}
