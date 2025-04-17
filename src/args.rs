use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "VARIANT")]
    pub run: Variant,
}

#[derive(Parser, Copy, Clone, ValueEnum, strum::Display)]
pub enum Variant {
    /// Make sample accounts
    MakeAccounts,
    /// Check balances for accounts in config
    Balances,
    /// Cross send between accounts in config
    CrossSend,
    /// Follow block creation and transfer sols
    FollowGeyser,
}
