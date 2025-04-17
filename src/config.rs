use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub balances: Vec<String>,
    pub send_from: Vec<CfgKeyPair>,
    pub send_to: Vec<String>,
    pub geyser_endpoint: String,
    pub geyser_key: String,
    pub on_block_from: CfgKeyPair,
    pub on_block_to: String,
    pub wallet_program: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct CfgKeyPair {
    pub key: String,
    pub sec: String,
}
