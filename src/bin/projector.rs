use clap::Parser;
use projector_rust::opts::Opts;
use anyhow::Result;
use projector_rust::config::Config;

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(());
}