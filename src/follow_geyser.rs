use crate::config::CfgKeyPair;
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::hash::Hash;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::system_transaction::transfer;
use anyhow::Result;
use futures::stream::StreamExt;
use log::{error, info};
use std::collections::HashMap;
use std::str::FromStr;
use tonic::transport::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;
use yellowstone_grpc_proto::prelude::*;

pub struct FollowGeyser {
    local_connection: RpcClient,
    endpoint: String,
    api_key: String,
    on_block_from: Keypair,
    on_block_to: Pubkey,
}

impl FollowGeyser {
    pub fn new(
        local_connection: RpcClient,
        endpoint: String,
        api_key: String,
        on_block_from: CfgKeyPair,
        on_block_to: String,
    ) -> Self {
        Self {
            local_connection,
            endpoint,
            api_key,
            on_block_from: Keypair::from_base58_string(&on_block_from.sec),
            on_block_to: Pubkey::from_str_const(&on_block_to),
        }
    }

    pub async fn follow(&self) -> Result<()> {
        let tls_config = ClientTlsConfig::new().with_native_roots();
        let builder = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .x_token(Some(self.api_key.clone()))?
            .tls_config(tls_config)?
            .max_decoding_message_size(1024 * 1024 * 1024);

        let mut client = builder.connect().await?;

        info!("Connect to stream");

        let mut blocks_meta: HashMap<String, SubscribeRequestFilterBlocksMeta> = HashMap::new();
        blocks_meta.insert("client".to_owned(), SubscribeRequestFilterBlocksMeta {});

        let (_, mut stream) = client
            .subscribe_with_request(Some(SubscribeRequest {
                slots: HashMap::new(),
                accounts: HashMap::new(),
                transactions: HashMap::new(),
                transactions_status: HashMap::new(),
                entry: HashMap::new(),
                blocks: HashMap::new(),
                blocks_meta,
                commitment: Some(CommitmentLevel::Finalized as i32),
                accounts_data_slice: Vec::new(),
                ping: None,
                from_slot: None,
            }))
            .await
            .map_err(backoff::Error::transient)?;

        while let Some(message) = stream.next().await {
            match message {
                Err(error) => {
                    error!("error: {error:?}");
                }
                Ok(msg) => {
                    if let Some(UpdateOneof::BlockMeta(data)) = msg.update_oneof {
                        info!("New block hash {}", data.blockhash);

                        match Hash::from_str(data.blockhash.as_str()) {
                            Ok(latest_hash) => self.send_sols(latest_hash).await?,
                            Err(error) => {
                                error!("On parse hash {}", error);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn send_sols(&self, _: Hash) -> Result<()> {
        // Денег тратить на тесты не будем, так что подменим хеш на локальный и отправим локально.
        let latest_hash = self.local_connection.get_latest_blockhash().await?;
        let tx = transfer(&self.on_block_from, &self.on_block_to, 50000, latest_hash);

        match self
            .local_connection
            .send_and_confirm_transaction(&tx)
            .await
        {
            Ok(_) => {
                info!("Some money send...")
            }
            Err(error) => {
                error!("On send - {:?}", error)
            }
        }

        Ok(())
    }
}
