use crate::args::{Args, Variant};
use crate::config::Config;
use clap::Parser;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::fs;

mod args;
mod balances;
mod config;
mod cross_send;
mod follow_geyser;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut logs_builder = Builder::new();

    logs_builder.filter_level(LevelFilter::Info);
    logs_builder.target(Target::Stdout);
    logs_builder.init();

    info!("Run in '{}' variant", args.run);

    let config_data = fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yaml::from_str(config_data.as_str())?;

    // TODO -

    match args.run {
        Variant::Balances => Ok(()),
        Variant::CrossSend => Ok(()),
        Variant::FollowGeyser => Ok(()),
        Variant::WalletTest => Ok(()),
    }
}
