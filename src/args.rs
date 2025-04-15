use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "VARIANT")]
    pub run: Variant,
}

#[derive(Parser, Copy, Clone, ValueEnum, strum::Display)]
pub enum Variant {
    Balances,
    CrossSend,
    FollowGeyser,
    WalletTest,
}
