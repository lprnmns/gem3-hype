use std::env;
use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::str::FromStr;
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct Config {
    pub wallet_private_key: String,
    pub wallet_address: String,
    pub master_address: Option<String>,
    pub trading: TradingParams,
    pub risk: RiskParams,
    pub is_dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct TradingParams {
    pub perp_symbol: String,
    pub spot_symbol: String,
    pub bps_threshold: Decimal,
    pub position_size_usd: Decimal,
    pub leverage: u32,
}

#[derive(Debug, Clone)]
pub struct RiskParams {
    pub max_position_size_usd: Decimal,
    pub stop_loss_bps: Decimal,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let wallet_private_key = env::var("HL_API_AGENT_PRIVATE_KEY")
            .context("HL_API_AGENT_PRIVATE_KEY must be set")?;
        
        let wallet_address = env::var("HL_API_AGENT_WALLET_ADDRESS")
            .context("HL_API_AGENT_WALLET_ADDRESS must be set")?;

        let master_address = env::var("HL_MASTER_ADDRESS").ok()
            .filter(|s| !s.is_empty());

        let perp_symbol = env::var("PERP_SYMBOL").unwrap_or_else(|_| "HYPE".to_string());
        let spot_symbol = env::var("SPOT_SYMBOL").unwrap_or_else(|_| "@107".to_string()); // HYPE spot asset ID might be needed

        let bps_threshold = Decimal::from_str(&env::var("BPS_THRESHOLD").unwrap_or_else(|_| "5.0".to_string()))?;
        let position_size_usd = Decimal::from_str(&env::var("POSITION_SIZE_USD").unwrap_or_else(|_| "20.0".to_string()))?;
        let leverage = env::var("LEVERAGE").unwrap_or_else(|_| "2".to_string()).parse()?;

        let max_position_size_usd = Decimal::from_str(&env::var("MAX_POSITION_SIZE_USD").unwrap_or_else(|_| "100.0".to_string()))?;
        let stop_loss_bps = Decimal::from_str(&env::var("STOP_LOSS_BPS").unwrap_or_else(|_| "50.0".to_string()))?;

        let is_dry_run = env::var("DRY_RUN").unwrap_or_else(|_| "true".to_string()) == "true";

        Ok(Config {
            wallet_private_key,
            wallet_address,
            master_address,
            trading: TradingParams {
                perp_symbol,
                spot_symbol,
                bps_threshold,
                position_size_usd,
                leverage,
            },
            risk: RiskParams {
                max_position_size_usd,
                stop_loss_bps,
            },
            is_dry_run,
        })
    }
}
