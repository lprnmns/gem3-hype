mod config;

use anyhow::Result;
use config::Config;
use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use log::{info, error};
use std::str::FromStr;
use hyperliquid_rust_sdk::types::H160;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Hyperliquid Arb Bot...");

    let config = Config::from_env()?;
    info!("Configuration loaded. Dry Run: {}", config.is_dry_run);
    info!("Agent Address: {}", config.wallet_address);
    if let Some(master) = &config.master_address {
        info!("Master Address: {}", master);
    }

    // TEST 1: Connect and Fetch Balances
    info!("--- TEST 1: Connecting to Info API ---");
    
    let mut info_client = InfoClient::new(BaseUrl::Mainnet).await?;
    
    let target_address = if let Some(master) = &config.master_address {
        H160::from_str(master)?
    } else {
        H160::from_str(&config.wallet_address)?
    };

    // Fetch Spot Balances
    info!("Fetching Spot Balances for {}...", target_address);
    let spot_state = info_client.spot_clearinghouse_state(target_address).await?;
    info!("Spot State retrieved successfully.");
    for balance in spot_state.balances {
        info!("Spot Balance - Coin: {}, Amount: {}", balance.coin, balance.total);
    }

    // Fetch Perp Balances
    info!("Fetching Perp Balances for {}...", target_address);
    let perp_state = info_client.user_state(target_address).await?;
    info!("Perp State retrieved successfully.");
    info!("Account Value: {}", perp_state.margin_summary.account_value);
    for position in perp_state.asset_positions {
        info!("Perp Position - Coin: {}, Size: {}, Entry: {}", position.position.coin, position.position.szi, position.position.entry_px);
    }

    info!("--- TEST 1 PASSED: Connection & Balance Check Successful ---");

    Ok(())
}
