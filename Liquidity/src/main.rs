use std::time::Duration;
use tokio;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::Rng;

mod pools;
mod liquidity;
mod proton_commands;

use proton_commands::prepare_deposit;
use pools::{get_pools, update_pools_periodically};
use liquidity::{add_liquidity_wrapper_snipsxp, add_liquidity_wrapper_xprxusdc};

static POOLS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static FIRST_RUN: AtomicBool = AtomicBool::new(true);

#[tokio::main]
async fn main() -> Result<()> {

    if FIRST_RUN.swap(false, Ordering::Relaxed) { 
        // Генерируем случайную задержку от 1 до 20 секунд при старте 
        let delay = rand::thread_rng().gen_range(1..=20); 
        println!("Waiting for {} seconds before starting the main loop...", delay); 
        tokio::time::sleep(Duration::from_secs(delay)).await; 
    }

    // Запускаем периодическое обновление информации о пулах каждую минуту
    tokio::spawn(async {
        if let Err(e) = update_pools_periodically(Duration::from_secs(60)).await {
            eprintln!("Error updating pools: {}", e);
        }
    });

    let xpr_amount = 111111.1111;
    let xusdc_amount = 1111.111111;

    loop {
        // Ждем, пока значения пулов не будут инициализированы
        while !POOLS_INITIALIZED.load(Ordering::Relaxed) {
            let (snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool) = get_pools()?;
            if snips_pool > 0.0 && xpr_pool > 0.0 && xusdc_pool > 0.0 && xpr_for_xusdc_pool > 0.0 {
                POOLS_INITIALIZED.store(true, Ordering::Relaxed);
                println!("Pools initialized: SNIPS={}, XPR={}, XUSDC={}, XPRforUSDC={}", snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool);
            } else {
                eprintln!("One or more pool values are zero. Waiting...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
        
        // Получаем информацию о пулах
        let (snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool) = get_pools()?;
        println!("Pool information: SNIPS={}, XPR={}, XUSDC={}, XPRforUSDC={}", snips_pool, xpr_pool, xusdc_pool, xpr_for_xusdc_pool);

        // Базовые значения для добавления ликвидности
        // let mut snips_amount = 0.1618;


        // Вычисляем значение SNIPS на основе отношения токенов в пуле SNIPSXP
        let snips_amount = xpr_amount * pools::calculate_token_ratio_snips_for_xpr(snips_pool, xpr_pool);

        // Вычисляем значение XPR на основе отношения токенов в пуле XPRUSDC
        let xpr_for_xusdc_amount = xusdc_amount * pools::calculate_token_ratio_xpr_for_xusdc(xusdc_pool, xpr_for_xusdc_pool);

        tokio::spawn(async {
            println!("Main.rs: Preparing deposit...");
            let _ = prepare_deposit(&[("6,XUSDC", "xtokens"), ("4,XPR", "eosio.token")]).await;
        });
        // Ждем 100 миллисекунд после запуска prepare_deposit
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Добавляем ликвидность в пулы параллельно
        let (snips_xpr_result, xpr_xusdc_result) = tokio::join!(
            tokio::spawn(async move{
                match add_liquidity_wrapper_snipsxp(snips_amount, xpr_amount).await {
                    Ok(_) => {
                        println!("Main.rs: Add liquidity to SNIPSXP pool successful.");
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("Error adding liquidity to SNIPSXP pool: {}", e);
                        Err(e)
                    }
                }
            }),
            tokio::spawn(async move{
                match add_liquidity_wrapper_xprxusdc(xpr_for_xusdc_amount, xusdc_amount).await {
                    Ok(_) => {
                        println!("Main.rs: Add liquidity to XPRXUSDC pool successful.");
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("Error adding liquidity to XPRXUSDC pool: {}", e);
                        Err(e)
                    }
                }
            })
        );

        // Проверяем результаты выполнения задач
        if let Err(e) = snips_xpr_result {
            eprintln!("Error spawning SNIPSXP liquidity task: {}", e);
        }
        if let Err(e) = xpr_xusdc_result {
            eprintln!("Error spawning XPRXUSDC liquidity task: {}", e);
        }
    }
}
