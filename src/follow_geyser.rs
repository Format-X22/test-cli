use anyhow::Result;
use tonic::transport::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;

pub struct FollowGeyser {
    endpoint: String,
    api_key: String,
}

impl FollowGeyser {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self { endpoint, api_key }
    }

    pub async fn follow(&self) -> Result<()> {
        let tls_config = ClientTlsConfig::new().with_native_roots();

        let mut builder = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .x_token(Some(self.endpoint.clone()))?
            .tls_config(tls_config)?
            .max_decoding_message_size(1024 * 1024 * 1024);

        let connection = builder.connect().await?;

        //connection;

        // TODO -

        Ok(())
    }
}
