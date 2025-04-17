use crate::args::{Args, Variant};
use crate::balances::Balances;
use crate::config::Config;
use crate::cross_send::CrossSend;
use crate::follow_geyser::FollowGeyser;
use crate::make_accounts::MakeAccounts;
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anyhow::{Error, Result, bail};
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
    let local_connection: RpcClient = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    match args.run {
        Variant::MakeAccounts => {
            let maker = MakeAccounts::new(local_connection);

            maker.make_sample().await
        }
        Variant::Balances => {
            if config.balances.len() < 1 {
                bail!("Empty balances config section!");
            }

            let balances = Balances::new(local_connection);

            balances.display_balances(config.balances).await
        }
        Variant::CrossSend => {
            let from = config.send_from;
            let to = config.send_to;

            if from.len() < 1 || to.len() < 1 {
                bail!("Empty cross sent config!")
            }

            if from.len() != to.len() {
                bail!("From address count not equal To address count, check config!")
            }

            let sender = CrossSend::new(local_connection);

            sender.send(from, to).await
        }
        Variant::FollowGeyser => {
            let follower = FollowGeyser::new(
                local_connection,
                config.geyser_endpoint,
                config.geyser_key,
                config.on_block_from,
                config.on_block_to,
            );

            follower.follow().await
        }
    }
}
