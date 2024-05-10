use anyhow::Result;
use tokio;
use std::time::Duration;
use rand::Rng;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

mod balance;
mod proton_commands;

use balance::get_balance;
use proton_commands::{prepare_deposit_wrapper, transfer_tokens, transfer_excess_snips, transfer_excess_xpr};

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

        // Выполняем transfer_excess_snips
        if let Some(updated_balances) = transfer_excess_snips().await? {
            balances = updated_balances;
        }

        // Выполняем transfer_excess_xpr
        if let Some(updated_balances) = transfer_excess_xpr().await? {
            balances = updated_balances;
        }

        // Получаем значения балансов для SNIPS, XPR и XUSDC
        let xpr_balance = balances.get("XPR").unwrap_or(&"0.0000".to_string()).parse::<f64>()?;
        let snips_balance = balances.get("SNIPS").unwrap_or(&"0.0000".to_string()).parse::<f64>()?;
        let xusdc_balance = balances.get("XUSDC").unwrap_or(&"0.000000".to_string()).parse::<f64>()?;

        // Определяем значение для передачи 0.2222 SNIPS и сохраняем остальные значения в переменных
        let fixed_snips_value = format!("{:.4} SNIPS", 0.2222);
        let snips_value = format!("{:.4} SNIPS", snips_balance);
        let xpr_transfer_value = format!("{:.4} XPR", xpr_balance);
        let xusdc_transfer_value = format!("{:.6} XUSDC", xusdc_balance);

        // Определяем значение для передачи SNIPS в зависимости от условия
        let snips_transfer_value = if snips_balance > 8500.0 && snips_balance < 60000.0 {
            fixed_snips_value.clone()
        } else {
            snips_value.clone()
        };
        
        // Выполняем prepare_deposit_wrapper без ожидания завершения
        tokio::spawn(async {
            println!("Выполняем prepare_deposit_wrapper для токенов XPR и SNIPS");
            let _ = tokio::join!(
                prepare_deposit_wrapper(&[("4,XPR", "eosio.token")]),
                prepare_deposit_wrapper(&[("4,SNIPS", "snipcoins")])
            );
        });
        
        tokio::spawn(async move{
            if xusdc_balance > 0.0 {
                println!("Выполняем prepare_deposit_wrapper для токена XUSDC");
                let _ = prepare_deposit_wrapper(&[("6,XUSDC", "xtokens")]);
            }
        });

        // Ждем 100 миллисекунд перед выполнением transfer_tokens
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Выполняем transfer_tokens асинхронно и независимо, если значения не нулевые
        let (snips_transfer, xpr_transfer, xusdc_transfer) = tokio::join!(
            async {
                if snips_balance > 0.0 {
                    let snips_transfer_value = snips_transfer_value.to_string();
                    Some(tokio::spawn(async move {
                        transfer_tokens("snipcoins", &snips_transfer_value, "proton.swaps", "deposit").await
                    }))
                } else {
                    None
                }
            },
            async {
                if xpr_balance > 0.0 {
                    let xpr_transfer_value = xpr_transfer_value.to_string();
                    Some(tokio::spawn(async move {
                        transfer_tokens("eosio.token", &xpr_transfer_value, "proton.swaps", "deposit").await
                    }))
                } else {
                    None
                }
            },
            async {
                if xusdc_balance > 0.0 {
                    let xusdc_transfer_value = xusdc_transfer_value.to_string();
                    Some(tokio::spawn(async move {
                        transfer_tokens("xtokens", &xusdc_transfer_value, "proton.swaps", "deposit").await
                    }))
                } else {
                    None
                }
            }
        );

        // Ожидаем завершения всех асинхронных задач
        if let Some(snips_task) = snips_transfer {
            if let Err(e) = snips_task.await {
                eprintln!("Error in SNIPS transfer: {}", e);
            }
        }

        if let Some(xpr_task) = xpr_transfer {
            if let Err(e) = xpr_task.await {
                eprintln!("Error in XPR transfer: {}", e);
            }
        }

        if let Some(xusdc_task) = xusdc_transfer {
            if let Err(e) = xusdc_task.await {
                eprintln!("Error in XUSDC transfer: {}", e);
            }
        }
    }
}
