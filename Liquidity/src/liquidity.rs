use anyhow::{Result, anyhow};
use regex::Regex;
use tokio::process::Command as TokioCommand;

// Импортируем необходимые функции и типы из других модулей
use crate::pools::get_pools;
use crate::pools::{calculate_token_ratio_xpr_for_snips, calculate_token_ratio_snips_for_xpr, calculate_token_ratio_xpr_for_xusdc, calculate_token_ratio_xusdc_for_xpr};

fn format_with_dot(value: &str) -> String {
    if value.contains('.') {
        value.to_string()
    } else {
        let value_int = value.parse::<u64>().unwrap_or(0);
        let value_str = value_int.to_string();
        let len = value_str.len();
        if len <= 4 {
            format!("0.{:0>4}", value_str)
        } else {
            let mut value_str_mutable = value_str;
            value_str_mutable.insert(len - 4, '.');
            value_str_mutable
        }
    }
}

fn format_with_dot_6_decimals(value: &str) -> String {
    if value.contains('.') {
        value.to_string()
    } else {
        let value_int = value.parse::<u64>().unwrap_or(0);
        let value_str = value_int.to_string();
        let len = value_str.len();
        if len <= 6 {
            format!("0.{:0>6}", value_str)
        } else {
            let mut value_str_mutable = value_str;
            value_str_mutable.insert(len - 6, '.');
            value_str_mutable
        }
    }
}

// Функция для добавления ликвидности (асинхронная версия)
async fn add_liquidity_async(
    lt_symbol: &str,
    token1_quantity: &str,
    token1_contract: &str,
    token2_quantity: &str,
    token2_contract: &str,
    token1_min_quantity: &str,
    token1_min_contract: &str,
    token2_min_quantity: &str,
    token2_min_contract: &str,
) -> Result<(String, String)> {
    println!("Managing liquidity...");
    println!("token1_quantity: {}", token1_quantity);
    println!("token1_min_quantity: {}", token1_min_quantity);
    println!("token2_quantity: {}", token2_quantity);
    println!("token2_min_quantity: {}", token2_min_quantity);
  
    let output = TokioCommand::new("sh")
        .arg("-c")
        .arg(format!(
            "proton action proton.swaps liquidityadd '{{\"owner\":\"panda4.gm\",\"lt_symbol\":\"{}\",\"add_token1\":{{\"quantity\":\"{}\",\"contract\":\"{}\"}},\"add_token2\":{{\"quantity\":\"{}\",\"contract\":\"{}\"}},\"add_token1_min\":{{\"quantity\":\"{}\",\"contract\":\"{}\"}},\"add_token2_min\":{{\"quantity\":\"{}\",\"contract\":\"{}\"}}}}' panda4.gm",
            lt_symbol, token1_quantity, token1_contract, token2_quantity, token2_contract, token1_min_quantity, token1_min_contract, token2_min_quantity, token2_min_contract
        ))
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok((stdout, stderr))
    } else {
        eprintln!("Failed to add liquidity. Error: {}", stderr);
        Err(anyhow!("Failed to add liquidity"))
    }
}

// Функция для добавления ликвидности в пул SNIPSXP (асинхронная версия)
pub async fn add_liquidity_wrapper_snipsxp(mut snips_amount: f64, mut xpr_amount: f64) -> Result<()> {
    // Получаем информацию о пулах
    let (snips_pool, xpr_pool, _, _) = match get_pools() {
        Ok(pools) => pools,
        Err(e) => {
            eprintln!("Failed to get pool information. Error: {}", e);
            return Err(e);
        }
    };

    // Проверяем, если snips_amount равно 0, то устанавливаем значение 1.1000
    if snips_amount == 0.0 {
        snips_amount = 1.1000;
    }

    // Проверяем, если xpr_amount равно 0, то устанавливаем значение 0.0220
    if xpr_amount == 0.0 {
        xpr_amount = 0.0220;
    }

    let mut snips_to_add_max = if snips_amount * calculate_token_ratio_xpr_for_snips(snips_pool, xpr_pool) > xpr_amount {
        xpr_amount * calculate_token_ratio_snips_for_xpr(snips_pool, xpr_pool)
    } else {
        snips_amount
    };

    let mut xpr_to_add_max = if xpr_amount * calculate_token_ratio_snips_for_xpr(snips_pool, xpr_pool) > snips_amount {
        snips_amount * calculate_token_ratio_xpr_for_snips(snips_pool, xpr_pool)
    } else {
        xpr_amount
    };

    let mut snips_to_add_max_str = format!("{:.4} SNIPS", snips_to_add_max);
    let mut xpr_to_add_max_str = format!("{:.4} XPR", xpr_to_add_max);
    let mut snips_to_add_min = format!("{:.4} SNIPS", snips_to_add_max * 0.9);
    let mut xpr_to_add_min = format!("{:.4} XPR", xpr_to_add_max * 0.9);

    println!("snips_amount: {}", snips_amount);
    println!("xpr_amount: {}", xpr_amount);
    println!("snips_to_add_max: {}", snips_to_add_max_str);
    println!("xpr_to_add_max: {}", xpr_to_add_max_str);
    println!("snips_to_add_min: {}", snips_to_add_min);
    println!("xpr_to_add_min: {}", xpr_to_add_min);

    // Добавляем ликвидность в пул SNIPSXP, если балансы достаточны
    if snips_amount > 0.0 && xpr_amount > 0.0 {
        let mut attempt = 0;
        let max_attempts = 4;

loop {
    attempt += 1;
    println!("Attempt #{} to add liquidity to SNIPSXP pool...", attempt);

    match add_liquidity_async(
        "8,SNIPSXP",
        &snips_to_add_max_str,
        "snipcoins",
        &xpr_to_add_max_str,
        "eosio.token",
        &snips_to_add_min,
        "snipcoins",
        &xpr_to_add_min,
        "eosio.token",
    ).await {
        Ok((stdout, _stderr)) => {
            println!("stdout: {}", stdout);

            let re_token1_low = Regex::new(r"Token 1 amount too low").unwrap();
            let re_token2_low = Regex::new(r"Token 2 amount too low").unwrap();
            let re_token1_low_min = Regex::new(r"Token 1 expected amount lower than minimum").unwrap();
            let re_token2_low_min = Regex::new(r"Token 2 expected amount lower than minimum").unwrap();
            let re_expected = Regex::new(r"Expected: (\d+(?:\.\d+)?)").unwrap();
            let re_balance_overdrawn = Regex::new(r"assertion failure with message: Balance overdrawn. Need (\d+(?:\.\d+)?) ([A-Z]+) but balance is (\d+(?:\.\d+)?) for symbol ([A-Z]+) contract ([a-z1-5]+)").unwrap();

            let mut updated = false;

            if re_token1_low.is_match(&stdout) || re_token1_low_min.is_match(&stdout) {
                println!("Matched re_token1_low or re_token1_low_min");
                if let Some(captures) = re_expected.captures(&stdout) {
                    let expected = captures.get(1).unwrap().as_str();
                    let expected_formatted = format_with_dot(expected);
                    snips_to_add_max_str = format!("{} SNIPS", expected_formatted);
                    snips_to_add_min = format!("{:.6} SNIPS", expected_formatted.parse::<f64>().unwrap() * 0.9);
                    xpr_to_add_max = expected_formatted.parse::<f64>().unwrap() * calculate_token_ratio_xpr_for_snips(snips_pool, xpr_pool);
                    xpr_to_add_max_str = format!("{} XPR", xpr_to_add_max);
                    xpr_to_add_min = format!("{} XPR", xpr_to_add_max * 0.9);
                    updated = true;
                }
            }

            if re_token2_low.is_match(&stdout) || re_token2_low_min.is_match(&stdout) {
                println!("Matched re_token2_low or re_token2_low_min");
                if let Some(captures) = re_expected.captures(&stdout) {
                    let expected = captures.get(1).unwrap().as_str();
                    let expected_formatted = format_with_dot(expected);
                    xpr_to_add_max_str = format!("{} XPR", expected_formatted);
                    xpr_to_add_min = format!("{} XPR", expected_formatted.parse::<f64>().unwrap() * 0.9);
                    snips_to_add_max = expected_formatted.parse::<f64>().unwrap() * calculate_token_ratio_snips_for_xpr(snips_pool, xpr_pool);
                    snips_to_add_max_str = format!("{:.6} SNIPS", snips_to_add_max);
                    snips_to_add_min = format!("{:.6} SNIPS", snips_to_add_max * 0.9);
                    updated = true;
                }
            }

            if re_balance_overdrawn.is_match(&stdout) {
                println!("Matched re_balance_overdrawn");
                if let Some(captures) = re_balance_overdrawn.captures(&stdout) {
                    let balance = captures.get(3).unwrap().as_str();
                    let balance_formatted = format_with_dot(balance);
                    let token_symbol = captures.get(4).unwrap().as_str();
                    let balance_value = balance_formatted.parse::<f64>().unwrap();

                    if balance_value < 0.0005 {
                        println!("Balance is less than 0.0005. Exiting the loop.");
                        break;
                    }

                    if token_symbol == "SNIPS" {
                        snips_to_add_max_str = format!("{} SNIPS", balance_formatted);
                        snips_to_add_min = format!("{:.4} SNIPS", balance_formatted.parse::<f64>().unwrap() * 0.9);
                        xpr_to_add_max = balance_formatted.parse::<f64>().unwrap() * calculate_token_ratio_xpr_for_snips(snips_pool, xpr_pool);
                        xpr_to_add_max_str = format!("{:.4} XPR", xpr_to_add_max);
                        xpr_to_add_min = format!("{:.4} XPR", xpr_to_add_max * 0.9);
                    } else if token_symbol == "XPR" {
                        xpr_to_add_max_str = format!("{} XPR", balance_formatted);
                        xpr_to_add_min = format!("{:.4} XPR", balance_formatted.parse::<f64>().unwrap() * 0.9);
                        snips_to_add_max = balance_formatted.parse::<f64>().unwrap() * calculate_token_ratio_snips_for_xpr(snips_pool, xpr_pool);
                        snips_to_add_max_str = format!("{:.4} SNIPS", snips_to_add_max);
                        snips_to_add_min = format!("{:.4} SNIPS", snips_to_add_max * 0.9);
                    }
                    updated = true;
                }
            }

            if !updated {
                println!("Liquidity added successfully to SNIPSXP pool. Output: {}", stdout);
                if !re_token1_low.is_match(&stdout) && !re_token2_low.is_match(&stdout) && !re_token1_low_min.is_match(&stdout) && !re_token2_low_min.is_match(&stdout) && !re_balance_overdrawn.is_match(&stdout) {
                    break; // Прерываем цикл, если ни одно из регулярных выражений не найдено
                }
            }
        }
        Err(e) => {
            eprintln!("add_liquidity failed for SNIPSXP pool. Error: {}", e);
            if attempt >= max_attempts {
                return Err(e);
            }
        }
    }

        if attempt >= max_attempts {
            eprintln!("Maximum number of attempts reached for SNIPSXP pool. Exiting...");
            return Err(anyhow!("Failed to add liquidity to SNIPSXP pool after {} attempts", max_attempts));
        }
    }

    } else {
        println!("Insufficient balance to add liquidity to SNIPSXP pool.");
    }
    
    Ok(())
}

// Функция для добавления ликвидности в пул XPRUSDC (асинхронная версия)
pub async fn add_liquidity_wrapper_xprxusdc(mut xpr_amount: f64, mut xusdc_amount: f64) -> Result<()> {
    // Получаем информацию о пулах
    let (_, _, xusdc_pool, xpr_for_xusdc_pool) = match get_pools() {
        Ok(pools) => pools,
        Err(e) => {
            eprintln!("Failed to get pool information. Error: {}", e);
            return Err(e);
        }
    };

    // Проверяем, если xusdc_amount равно 0, то устанавливаем значение 0.000110
    if xusdc_amount == 0.0 {
        xusdc_amount = 0.000110;
    }

    // Проверяем, если xpr_amount равно 0, то устанавливаем значение 0.0220
    if xpr_amount == 0.0 {
        xpr_amount = 0.0220;
    }

    let mut xusdc_to_add_max = if xusdc_amount * calculate_token_ratio_xpr_for_xusdc(xusdc_pool, xpr_for_xusdc_pool) > xpr_amount {
        xpr_amount * calculate_token_ratio_xusdc_for_xpr(xusdc_pool, xpr_for_xusdc_pool)
    } else {
        xusdc_amount
    };

    let mut xpr_to_add_max = if xpr_amount * calculate_token_ratio_xusdc_for_xpr(xusdc_pool, xpr_for_xusdc_pool) > xusdc_amount {
        xusdc_amount * calculate_token_ratio_xpr_for_xusdc(xusdc_pool, xpr_for_xusdc_pool)
    } else {
        xpr_amount
    };

    let mut xusdc_to_add_max_str = format!("{:.6} XUSDC", xusdc_to_add_max);
    let mut xpr_to_add_max_str = format!("{:.4} XPR", xpr_to_add_max);
    let mut xusdc_to_add_min = format!("{:.6} XUSDC", xusdc_to_add_max * 0.9);
    let mut xpr_to_add_min = format!("{:.4} XPR", xpr_to_add_max * 0.9);

    println!("xusdc_amount: {}", xusdc_amount);
    println!("xpr_amount: {}", xpr_amount);
    println!("xusdc_to_add_max: {}", xusdc_to_add_max_str);
    println!("xpr_to_add_max: {}", xpr_to_add_max_str);
    println!("xusdc_to_add_min: {}", xusdc_to_add_min);
    println!("xpr_to_add_min: {}", xpr_to_add_min);

    // Добавляем ликвидность в пул XPRUSDC, если балансы достаточны
    if xusdc_amount > 0.0 && xpr_amount > 0.0 {
        let mut attempt = 0;
        let max_attempts = 4;

        loop {
            attempt += 1;
            println!("Attempt #{} to add liquidity to XPRUSDC pool...", attempt);

            match add_liquidity_async(
                "8,XPRUSDC",
                &xpr_to_add_max_str,
                "eosio.token",
                &xusdc_to_add_max_str,
                "xtokens",
                &xpr_to_add_min,
                "eosio.token",
                &xusdc_to_add_min,
                "xtokens",
            ).await {
                Ok((stdout, _stderr)) => {
                    println!("stdout: {}", stdout);

                    let re_token1_low = Regex::new(r"Token 1 amount too low").unwrap();
                    let re_token2_low = Regex::new(r"Token 2 amount too low").unwrap();
                    let re_token1_low_min = Regex::new(r"Token 1 expected amount lower than minimum").unwrap();
                    let re_token2_low_min = Regex::new(r"Token 2 expected amount lower than minimum").unwrap();
                    let re_expected = Regex::new(r"Expected: (\d+(?:\.\d+)?)").unwrap();
                    let re_balance_overdrawn = Regex::new(r"assertion failure with message: Balance overdrawn. Need (\d+(?:\.\d+)?) ([A-Z]+) but balance is (\d+(?:\.\d+)?) for symbol ([A-Z]+) contract ([a-z1-5]+)").unwrap();

                    let mut updated = false;

                    if re_token1_low.is_match(&stdout) || re_token1_low_min.is_match(&stdout) {
                        println!("Matched re_token1_low or re_token1_low_min");
                        if let Some(captures) = re_expected.captures(&stdout) {
                            let expected = captures.get(1).unwrap().as_str();
                            let expected_formatted = format_with_dot(expected);
                            xpr_to_add_max_str = format!("{} XPR", expected_formatted);
                            xpr_to_add_min = format!("{:.4} XPR", expected_formatted.parse::<f64>().unwrap() * 0.9);
                            xusdc_to_add_max = expected_formatted.parse::<f64>().unwrap() * calculate_token_ratio_xusdc_for_xpr(xusdc_pool, xpr_for_xusdc_pool);
                            xusdc_to_add_max_str = format!("{:.6} XUSDC", xusdc_to_add_max);
                            xusdc_to_add_min = format!("{:.6} XUSDC", xusdc_to_add_max * 0.9);
                            updated = true;
                        }
                    }

                    if re_token2_low.is_match(&stdout) || re_token2_low_min.is_match(&stdout) {
                        println!("Matched re_token2_low or re_token2_low_min");
                        if let Some(captures) = re_expected.captures(&stdout) {
                            let expected = captures.get(1).unwrap().as_str();
                            let expected_formatted = format_with_dot_6_decimals(expected);
                            xusdc_to_add_max_str = format!("{:.6} XUSDC", expected_formatted);
                            xusdc_to_add_min = format!("{:.6} XUSDC", expected_formatted.parse::<f64>().unwrap() * 0.9);
                            xpr_to_add_max = expected_formatted.parse::<f64>().unwrap() * calculate_token_ratio_xpr_for_xusdc(xusdc_pool, xpr_for_xusdc_pool);
                            xpr_to_add_max_str = format!("{:.4} XPR", xpr_to_add_max);
                            xpr_to_add_min = format!("{:.4} XPR", xpr_to_add_max * 0.9);
                            updated = true;
                        }
                    }

                    if re_balance_overdrawn.is_match(&stdout) {
                        println!("Matched re_balance_overdrawn");
                        if let Some(captures) = re_balance_overdrawn.captures(&stdout) {
                            let balance = captures.get(3).unwrap().as_str();
                            let token_symbol = captures.get(4).unwrap().as_str();
                    
                            if token_symbol == "XPR" {
                                let balance_formatted = format_with_dot(balance);
                                let balance_value = balance_formatted.parse::<f64>().unwrap();
                    
                                if balance_value < 0.0005 {
                                    println!("XPR balance is less than 0.0005. Exiting the loop.");
                                    break;
                                }
                    
                                xpr_to_add_max_str = format!("{} XPR", balance_formatted);
                                xpr_to_add_min = format!("{:.4} XPR", balance_value * 0.9);
                                xusdc_to_add_max = balance_value * calculate_token_ratio_xusdc_for_xpr(xusdc_pool, xpr_for_xusdc_pool);
                                xusdc_to_add_max_str = format!("{:.6} XUSDC", xusdc_to_add_max);
                                xusdc_to_add_min = format!("{:.6} XUSDC", xusdc_to_add_max * 0.9);
                            } else if token_symbol == "XUSDC" {
                                let balance_formatted = format_with_dot_6_decimals(balance);
                                let balance_value = balance_formatted.parse::<f64>().unwrap();
                    
                                if balance_value < 0.000005 {
                                    println!("XUSDC balance is less than 0.000005. Exiting the loop.");
                                    break;
                                }
                    
                                xusdc_to_add_max_str = format!("{} XUSDC", balance_formatted);
                                xusdc_to_add_min = format!("{:.6} XUSDC", balance_value * 0.9);
                                xpr_to_add_max = balance_value * calculate_token_ratio_xpr_for_xusdc(xusdc_pool, xpr_for_xusdc_pool);
                                xpr_to_add_max_str = format!("{:.4} XPR", xpr_to_add_max);
                                xpr_to_add_min = format!("{:.4} XPR", xpr_to_add_max * 0.9);
                            }
                            updated = true;
                        }
                    }

                    if !updated {
                        println!("Liquidity added successfully to XPRUSDC pool. Output: {}", stdout);
                        if !re_token1_low.is_match(&stdout) && !re_token2_low.is_match(&stdout) && !re_token1_low_min.is_match(&stdout) && !re_token2_low_min.is_match(&stdout) && !re_balance_overdrawn.is_match(&stdout) {
                            break; // Прерываем цикл, если ни одно из регулярных выражений не найдено
                        }
                    }
                }
                Err(e) => {
                    eprintln!("add_liquidity failed for XPRUSDC pool. Error: {}", e);
                    if attempt >= max_attempts {
                        return Err(e);
                    }
                }
            }

            if attempt >= max_attempts {
                eprintln!("Maximum number of attempts reached for XPRUSDC pool. Exiting...");
                return Err(anyhow!("Failed to add liquidity to XPRUSDC pool after {} attempts", max_attempts));
            }
        }
    } else {
        println!("Insufficient balance to add liquidity to XPRUSDC pool.");
    }

    Ok(())
}