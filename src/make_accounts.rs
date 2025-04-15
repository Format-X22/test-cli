use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::native_token::LAMPORTS_PER_SOL;
use anchor_client::solana_sdk::signature::Signature;
use anchor_client::solana_sdk::{signature::Keypair, signer::Signer};
use anchor_lang::prelude::*;
use anyhow::Result;
use bs58::encode;
use log::info;
use std::time::Duration;
use tokio::time::sleep;

pub struct MakeAccounts {
    connection: RpcClient,
}

impl MakeAccounts {
    pub fn new(connection: RpcClient) -> Self {
        Self { connection }
    }

    pub async fn make_sample(&self) -> Result<()> {
        info!("Make 5 sample accounts with balances...\n\n");

        let mut accounts: Vec<Keypair> = vec![];

        for i in 0..5 {
            let acc = Keypair::new();

            info!("Acc {i} KEY = {}", acc.pubkey());
            info!(
                "Acc {i} SEC = {}",
                encode(acc.secret().to_bytes()).into_string()
            );

            accounts.push(acc);
        }

        info!("Request airdrops...");

        for acc in accounts {
            let sig = self
                .connection
                .request_airdrop(&acc.pubkey(), LAMPORTS_PER_SOL)
                .await?;

            while !self.connection.confirm_transaction(&sig).await? {
                sleep(Duration::from_millis(100)).await;
            }

            let balance = self.connection.get_balance(&acc.pubkey()).await?;

            info!("Balance for {} is {}", acc.pubkey(), balance);
        }

        info!("Dropped!");

        info!(
            "\n\nSample account created and have some balances. Can be copied to config for next steps."
        );

        Ok(())
    }
}
