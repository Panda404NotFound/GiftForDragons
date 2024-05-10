use std::process::Command;
use std::sync::RwLock;
use anyhow::{Result, anyhow};
use serde::Deserialize;
use lazy_static::lazy_static;
use tokio::time::Duration;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::Instant;

static SNIPS_POOL: AtomicU64 = AtomicU64::new(0);
static USDC_POOL: AtomicU64 = AtomicU64::new(0);
static XPRFORUSDC_POOL: AtomicU64 = AtomicU64::new(0);
static XPR_POOL: AtomicU64 = AtomicU64::new(0);
static LAST_UPDATE: AtomicU64 = AtomicU64::new(0);
static POOLS_UPDATED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Deserialize)]
struct Response {
    rows: Vec<Pool>,
}

#[derive(Debug, Deserialize)]
struct Pool {
    lt_symbol: String,
    pool1: PoolToken,
    pool2: PoolToken,
}

#[derive(Debug, Deserialize)]
struct PoolToken {
    quantity: String,
}

lazy_static! {
    static ref POOLS: RwLock<Option<(f64, f64, std::time::Instant)>> = RwLock::new(None);
}

// Функция для получения информации о пулах
fn fetch_pools() -> Result<(f64, f64, f64, f64)> {
    let mut attempts = 0;
        loop {
            let output = Command::new("sh")
            .arg("-c")
            .arg("proton table proton.swaps pools")
            .output()?;

        if output.status.success() {

            let output_str = String::from_utf8_lossy(&output.stdout);
            let response: Response = serde_json::from_str(&output_str)?;
            let pools = response.rows;

            let xprxusdc_pool = pools
            .iter()
            .find(|pool| pool.lt_symbol == "8,XPRUSDC")
            .ok_or_else(|| anyhow!("XPRUSDC pool not found"))?;

            let snipsxp_pool = pools
                .iter()
                .find(|pool| pool.lt_symbol == "8,SNIPSXP")
                .ok_or_else(|| anyhow!("SNIPSXP pool not found"))?;

            let snips_amount = snipsxp_pool.pool1.quantity.split_whitespace().next().unwrap_or("0.0000");
            let xpr_amount = snipsxp_pool.pool2.quantity.split_whitespace().next().unwrap_or("0.0000");

            let _snips_pool: f64 = snips_amount.parse()?;
            let _xpr_pool: f64 = xpr_amount.parse()?;

            let xpr_for_xusdc_amount = xprxusdc_pool.pool1.quantity.split_whitespace().next().unwrap_or("0.0000");
            let xusdc_amount = xprxusdc_pool.pool2.quantity.split_whitespace().next().unwrap_or("0.000000");

            let snips_pool: f64 = snips_amount.parse()?;
            let xpr_pool: f64 = xpr_amount.parse()?;

            let xusdc_pool: f64 = xusdc_amount.parse()?;
            let xpr_for_xusdc_pool: f64 = xpr_for_xusdc_amount.parse()?;

            return Ok((snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool));
        } else {
            eprintln!("Error in fetch_pools: {}", String::from_utf8_lossy(&output.stderr));
            attempts += 1;
            if attempts >= 5 {
                return Err(anyhow::anyhow!("Failed to fetch_pools information after 5 attempts"));
            }
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }
}

// Функция для получения информации о пулах с кешированием
pub fn get_pools() -> Result<(f64, f64, f64, f64)> {
    let now = Instant::now().elapsed().as_secs();
    if now - LAST_UPDATE.load(Ordering::Relaxed) >= 60 && !POOLS_UPDATED.load(Ordering::Relaxed) {
        let fetched_pools = fetch_pools()?;
        SNIPS_POOL.store(fetched_pools.0 as u64, Ordering::Relaxed);
        USDC_POOL.store(fetched_pools.1 as u64, Ordering::Relaxed);
        XPRFORUSDC_POOL.store(fetched_pools.2 as u64, Ordering::Relaxed);
        XPR_POOL.store(fetched_pools.3 as u64, Ordering::Relaxed);
        POOLS_UPDATED.store(true, Ordering::Relaxed);
        LAST_UPDATE.store(now, Ordering::Relaxed);
    }
    let snips_pool = SNIPS_POOL.load(Ordering::Relaxed) as f64;
    let xpr_pool = XPR_POOL.load(Ordering::Relaxed) as f64;
    let xusdc_pool = USDC_POOL.load(Ordering::Relaxed) as f64;
    let xpr_for_xusdc_pool = XPRFORUSDC_POOL.load(Ordering::Relaxed) as f64;
    Ok((snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool))
}

// Функция для периодического обновления информации о пулах
pub async fn update_pools_periodically(interval: Duration) -> Result<(), anyhow::Error> {
    loop {
        let result = tokio::task::spawn_blocking(|| fetch_pools()).await?;
        match result {
            Ok((snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool)) => {
                SNIPS_POOL.store(snips_pool as u64, Ordering::Relaxed);
                XPR_POOL.store(xpr_pool as u64, Ordering::Relaxed);
                USDC_POOL.store(xusdc_pool as u64, Ordering::Relaxed);
                XPRFORUSDC_POOL.store(xpr_for_xusdc_pool as u64, Ordering::Relaxed);
                POOLS_UPDATED.store(true, Ordering::Relaxed);
                println!("Pools updated successfully: SNIPS={}, XPR={}, XUSDC={}, XPRforUSDC={}", snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool);
            }
            Err(e) => {
                eprintln!("Error updating pools: {}", e);
            }
        }
        tokio::time::sleep(interval).await;

    }
}

// Добавим функцию для расчета соотношения токенов в пуле из Liquidity.rs
pub fn calculate_token_ratio_snips_for_xpr(snips_pool: f64, xpr_pool: f64) -> f64 {
    snips_pool / xpr_pool
}

pub fn calculate_token_ratio_xpr_for_snips(snips_pool: f64, xpr_pool: f64) -> f64 {
    xpr_pool / snips_pool
}

pub fn calculate_token_ratio_xusdc_for_xpr(xusdc_pool: f64, xpr_for_xusdc_pool: f64) -> f64 {
    xusdc_pool / xpr_for_xusdc_pool
}

pub fn calculate_token_ratio_xpr_for_xusdc(xusdc_pool: f64, xpr_for_xusdc_pool: f64) -> f64 {
    xpr_for_xusdc_pool / xusdc_pool
}
