use clap::Parser;
use anyhow::Result;
use projector_rust::opts::Opts;
use projector_rust::config::{Config, Operation};
use projector_rust::projector::Projector;

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut projector = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = projector.get_all_value();
            let value = serde_json::to_string(&value)?;
            println!("{}", value);
        },
        Operation::Print(Some(k)) => {
            projector.get_value(&k).map(|value| {
                println!("{}", value);
            });
        },
        Operation::Add(k, v) => {
            projector.set_value(k, v);
            projector.save()?;
        },
        Operation::Remove(k) => {
            projector.remove_value(&k);
            projector.save()?;
        }
    };

    return Ok(());
}