use crate::config::CfgKeyPair;
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{Keypair, Signature, Signer};
use anchor_client::solana_sdk::system_transaction::transfer;
use anyhow::Result;
use chrono::Utc;
use log::info;
use std::time::Duration;
use tokio::time::sleep;

type Pairs = Vec<(Keypair, Pubkey)>;

pub struct CrossSend {
    connection: RpcClient,
}

impl CrossSend {
    pub fn new(connection: RpcClient) -> Self {
        Self { connection }
    }

    pub async fn send(&self, from: Vec<CfgKeyPair>, to: Vec<String>) -> Result<()> {
        let pairs = self.extract_pairs(from, to);

        self.print_current_balances(&pairs).await?;

        info!("Start cross send...");

        let start_time = Utc::now();
        let sigs = self.send_transactions(&pairs).await?;

        self.wait_confirmations(&sigs).await?;

        let end_time = Utc::now();

        info!(
            "Execution time = {}ms",
            end_time.timestamp_millis() - start_time.timestamp_millis()
        );

        self.print_statuses(&sigs).await?;
        self.print_current_balances(&pairs).await?;

        Ok(())
    }

    fn extract_pairs(&self, from: Vec<CfgKeyPair>, to: Vec<String>) -> Pairs {
        let mut result = vec![];

        for i in 0..from.len() {
            let from_acc = from.get(i).expect("On get from");
            let to_acc = to.get(i).expect("On get to");

            let from = Keypair::from_base58_string(from_acc.sec.as_str());
            let to = Pubkey::from_str_const(to_acc.as_str());

            result.push((from, to));
        }

        result
    }

    async fn print_current_balances(&self, pairs: &Pairs) -> Result<()> {
        for pair in pairs {
            let from_key = pair.0.pubkey();
            let from_value = self.connection.get_balance(&from_key).await?;
            let to_key = pair.1;
            let to_value = self.connection.get_balance(&to_key).await?;

            info!(
                "Balances for pair: \n{} = {}\n {} = {}",
                from_key, from_value, to_key, to_value
            );
        }

        Ok(())
    }

    async fn send_transactions(&self, pairs: &Pairs) -> Result<Vec<Signature>> {
        let mut result = vec![];
        let latest_hash = self.connection.get_latest_blockhash().await?;

        for pair in pairs {
            let tx = transfer(&pair.0, &pair.1, 50000, latest_hash);
            let sig = self.connection.send_transaction(&tx).await?;

            result.push(sig);
        }

        Ok(result)
    }

    async fn wait_confirmations(&self, sigs: &Vec<Signature>) -> Result<()> {
        for sig in sigs {
            while !self.connection.confirm_transaction(sig).await? {
                sleep(Duration::from_millis(100)).await;
            }
        }

        Ok(())
    }

    async fn print_statuses(&self, sigs: &Vec<Signature>) -> Result<()> {
        for sig in sigs {
            let status = self.connection.get_signature_status(sig).await?;

            match status {
                None => info!("Transaction {} not found!", sig),
                Some(result) => match result {
                    Err(fail) => info!("Transaction {} failed with {}", sig, fail),
                    Ok(_) => info!("Transaction {} is successful!", sig),
                },
            }
        }

        Ok(())
    }
}
