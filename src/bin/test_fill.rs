

use anyhow::Result;
use gemini3_hype::Config;
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient, InfoClient};
use hyperliquid_rust_sdk::types::{
    exchange::request::{Limit, OrderRequest, Tif},
    H160,
};
use log::{info, error, warn};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Test 3: Real Fill Test (Market Buy)...");

    let config = Config::from_env()?;
    
    // 1. Initialize Clients
    let mut info_client = InfoClient::new(BaseUrl::Mainnet).await?;
    let exchange_client = ExchangeClient::new(
        BaseUrl::Mainnet,
        config.wallet_private_key.clone(),
        Some(H160::from_str(&config.wallet_address)?),
        None,
    ).await?;

    info!("Clients initialized.");

    // 2. Get Current Price for HYPE Spot
    let spot_symbol = "HYPE"; // Ensure this matches the API symbol
    info!("Fetching L2 Book for {}...", spot_symbol);
    
    // Note: In HL SDK, we might need to look up the asset ID or use the symbol directly depending on the method.
    // For L2 Snapshot, we usually need the coin name.
    let l2_snapshot = info_client.l2_snapshot(spot_symbol.to_string()).await?;
    
    let best_ask = l2_snapshot.levels.first().map(|l| l.px).ok_or(anyhow::anyhow!("No asks in book"))?;
    info!("Best Ask Price: ${}", best_ask);

    // 3. Calculate Size for ~12 USDC
    let target_usd_size = 12.0;
    let raw_sz = target_usd_size / best_ask;
    // Round to 2 decimal places (or whatever the lot size is, usually safe to round down slightly)
    let sz = (raw_sz * 100.0).floor() / 100.0;
    
    info!("Target Size: {} HYPE (~${})", sz, target_usd_size);

    if sz <= 0.0 {
        error!("Calculated size is too small!");
        return Ok(());
    }

    // 4. Execute Market Buy
    // Using Limit IOC with high price acts as Market Buy but safer
    let limit_px = best_ask * 1.05; // 5% slippage tolerance
    
    info!("Placing Aggressive Limit Buy (Market-like): Buy {} HYPE @ < ${}", sz, limit_px);
    
    let order_request = OrderRequest::Limit(Limit {
        coin: spot_symbol.to_string(),
        is_buy: true,
        sz,
        limit_px,
        order_type: Tif::Ioc, // Immediate or Cancel (Fills what it can immediately, cancels rest)
        reduce_only: false,
        cloid: None,
    });

    let response = exchange_client.order(order_request, None).await?;
    
    match response {
        Ok(status) => {
            info!("Order Sent! Response: {:?}", status);
            // Check if it was filled (status usually contains filling info or oid)
            // For now, we assume success if no error, user will verify in UI.
            info!("Check your Hyperliquid UI for the fill.");
        },
        Err(e) => {
            error!("Order Failed: {:?}", e);
        }
    }

    info!("--- TEST 3 COMPLETED ---");
    Ok(())
}
