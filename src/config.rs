use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub balances: Vec<String>,
    pub send_from: Vec<CfgKeyPair>,
    pub send_to: Vec<String>,
    pub on_block_from: String,
    pub on_block_to: String,
    pub wallet_program: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct CfgKeyPair {
    pub key: String,
    pub sec: String,
}
