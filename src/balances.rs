use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use log::info;

pub struct Balances {
    connection: RpcClient,
}

impl Balances {
    pub fn new(connection: RpcClient) -> Self {
        Self { connection }
    }

    pub async fn display_balances(&self, accounts: Vec<String>) -> Result<()> {
        info!("Print balances from config section...\n");

        for acc in accounts {
            let key = Pubkey::from_str_const(acc.as_str());
            let balance = self.connection.get_balance(&key).await?;

            info!("Balance for {} is {}", acc, balance);
        }

        Ok(())
    }
}
