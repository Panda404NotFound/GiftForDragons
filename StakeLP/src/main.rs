use tokio;
use anyhow::Result;
use rand::Rng;
use std::time::Duration;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

mod proton_commands;
mod balance;

use proton_commands::open_stake;
use proton_commands::{handle_snipsxp_staking, handle_xprusdc_staking};
use balance::get_balance;

static FIRST_RUN: AtomicBool = AtomicBool::new(true);

#[tokio::main]
async fn main() -> Result<()> {

    if FIRST_RUN.swap(false, Ordering::Relaxed) { 
        // Генерируем случайную задержку от 1 до 20 секунд при старте 
        let delay = rand::thread_rng().gen_range(1..=20); 
        println!("Waiting for {} seconds before starting the main loop...", delay); 
        tokio::time::sleep(Duration::from_secs(delay)).await; 
    }

    loop {
        // Получаем баланс токенов
        let mut balances = get_balance()?;
        println!("Token balances: {:?}", balances);

        // Проверяем, если все значения баланса равны нулю
        let all_balances_zero = balances.values().all(|balance| balance == "0.00000000");

        if all_balances_zero {
            // Повторяем запрос баланса на 3 попытки
            let max_retries = 3;
            let mut retry_count = 0;

            while retry_count < max_retries {
                balances = get_balance()?;
                if !balances.values().all(|balance| balance == "0.00000000") {
                    break;
                }
                retry_count += 1;
            }

            // Если после 3 попыток баланс все еще равен нулю, пропускаем итерацию
            if retry_count == max_retries {
                continue;
            }
        }

        // Получаем значения балансов для SNIPSXP и XPRUSDC
        let xprsnips_balance = balances.get("SNIPSXP").unwrap_or(&"0.00000000".to_string()).parse::<f64>()?;
        let xprxusdc_balance = balances.get("XPRUSDC").unwrap_or(&"0.00000000".to_string()).parse::<f64>()?;

        // Открываем стейкинг для пулов, если баланс больше нуля, без ожидания завершения
        if xprsnips_balance > 0.0 {
            println!("Opening staking for SNIPSXP");
            let _ = open_stake(&["SNIPSXP"]);
        }
        if xprxusdc_balance > 0.0 {
            println!("Opening staking for XPRUSDC ");
            let _ = open_stake(&["XPRUSDC"]);
        }

        // Добавляем задержку после открытия стейкинга
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Сохраняем форматированные строки перед использованием
        let formatted_snipsxp = format!("{:.8} SNIPSXP", xprsnips_balance);
        let formatted_xprusdc = format!("{:.8} XPRUSDC", xprxusdc_balance);

        // Запускаем асинхронную обработку стейкинга только для пулов с балансом выше 0
        
        let (snipsxp_staking_handle, xprusdc_staking_handle) = tokio::join!(
            async {
                if xprsnips_balance > 0.0 {
                    Some(tokio::spawn(handle_snipsxp_staking(formatted_snipsxp)))
                } else {
                    None
                }
            },
            async {
                if xprxusdc_balance > 0.0 {
                    Some(tokio::spawn(handle_xprusdc_staking(formatted_xprusdc)))
                } else {
                    None
                }
            }
        );

        // Ожидаем завершения только тех задач, которые были запущены
        if let Some(handle) = snipsxp_staking_handle {
            match handle.await {
                Ok(result) => {
                    if let Err(e) = result {
                        eprintln!("Error in SNIPSXP staking task: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error in SNIPSXP staking task: {}", e);
                }
            }
        }

        if let Some(handle) = xprusdc_staking_handle {
            match handle.await {
                Ok(result) => {
                    if let Err(e) = result {
                        eprintln!("Error in XPRUSDC staking task: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error in XPRUSDC staking task: {}", e);
                }
            }
        }
    }
}    
