use crate::args::{Args, Variant};
use crate::balances::Balances;
use crate::config::Config;
use crate::make_accounts::MakeAccounts;
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anyhow::{Error, Ok, Result};
use clap::Parser;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::fs;

mod args;
mod balances;
mod config;
mod cross_send;
mod follow_geyser;
mod make_accounts;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut logs_builder = Builder::new();

    logs_builder.filter_level(LevelFilter::Info);
    logs_builder.target(Target::Stdout);
    logs_builder.init();

    info!("Run in '{}' variant", args.run);

    let config_data = fs::read_to_string("./config.yaml")?;
    let config: Config = serde_yaml::from_str(config_data.as_str())?;
    let connection: RpcClient = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    // TODO -

    match args.run {
        Variant::MakeAccounts => {
            let maker = MakeAccounts::new(connection);

            maker.make_sample().await
        }
        Variant::Balances => {
            let balances = Balances::new(connection);

            balances.display_balances(config.balances).await
        }
        Variant::CrossSend => Ok(()),
        Variant::FollowGeyser => Ok(()),
        Variant::WalletTest => Ok(()),
    }
}
