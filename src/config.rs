use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub balances: Vec<String>,
    pub send_from: Vec<String>,
    pub send_to: Vec<String>,
    pub on_block_from: String,
    pub on_block_to: String,
    pub wallet_program: String,
}
