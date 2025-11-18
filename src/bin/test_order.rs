mod config;

use anyhow::Result;
use config::Config;
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient};
use hyperliquid_rust_sdk::types::{
    exchange::request::{Limit, OrderRequest, Tif},
    H160,
};
use log::{info, error};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Test 2: Order Signing & Placement...");

    let config = Config::from_env()?;
    
    // Initialize Exchange Client
    let exchange_client = ExchangeClient::new(
        BaseUrl::Mainnet,
        config.wallet_private_key.clone(),
        Some(H160::from_str(&config.wallet_address)?),
        None,
    ).await?;

    info!("Exchange Client initialized.");

    // Define Order Parameters (Safe Test Order)
    // Buy HYPE Spot at a very low price (e.g., $1.0) to ensure it doesn't fill immediately
    // WARNING: Ensure this price is far below market price!
    let asset_name = "HYPE"; // Spot Asset Name
    let limit_px = 1.0; 
    let sz = 1.0; // 1 HYPE
    
    info!("Preparing Test Order: Buy {} {} @ ${}", sz, asset_name, limit_px);

    // 1. Place Order
    let order_request = OrderRequest::Limit(Limit {
        coin: asset_name.to_string(),
        is_buy: true,
        sz,
        limit_px,
        order_type: Tif::Gtc, // Good Till Cancelled
        reduce_only: false,
        cloid: None,
    });

    info!("Sending Order...");
    let response = exchange_client.order(order_request, None).await?;
    
    match response {
        Ok(status) => {
            info!("Order Placed Successfully! Response: {:?}", status);
            
            // Wait a bit
            sleep(Duration::from_secs(2)).await;

            // 2. Cancel Order (Cleanup)
            // Note: In a real scenario, we would need the OID from the response to cancel specifically.
            // For this test, we can cancel all open orders for this asset to be safe/clean.
            info!("Cancelling all open orders for {}...", asset_name);
            let cancel_response = exchange_client.cancel_all(asset_name.to_string(), None).await?;
            match cancel_response {
                Ok(cancel_status) => info!("Orders Cancelled: {:?}", cancel_status),
                Err(e) => error!("Failed to cancel orders: {:?}", e),
            }
        },
        Err(e) => {
            error!("Order Placement Failed: {:?}", e);
            return Err(anyhow::anyhow!("Order placement failed"));
        }
    }

    info!("--- TEST 2 PASSED: Order Signing & Placement Successful ---");
    Ok(())
}
