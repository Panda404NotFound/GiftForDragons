use std::process::Command;
use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;
use std::thread;
use std::time::Duration;

pub fn get_balance() -> Result<HashMap<String, String>> {
    let max_retries = 5;
    let mut retry_count = 0;

    loop {
        let output = Command::new("sh")
        .arg("-c")
        .arg("proton account panda4.gm -t")
        .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut balances = HashMap::new();

            let re_token = Regex::new(r"(\d+(?:\.\d+)?)\s+(\w+)\s+-\s+\w+")?;
            for line in output_str.lines() {
                if let Some(caps) = re_token.captures(line) {
                    let amount = &caps[1];
                    let symbol = &caps[2];
                    balances.insert(symbol.to_string(), amount.to_string());
                }
            }

            return Ok(balances);
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            let re_abort_error = Regex::new(r"AbortError: The user aborted a request")?;
            let re_failed_to_get_balance = Regex::new(r"Failed to get balance for account: panda4.gm")?;

            if (re_abort_error.is_match(&error_message) || re_failed_to_get_balance.is_match(&error_message))
                && retry_count < max_retries
            {
                retry_count += 1;
                eprintln!(
                    "Error getting balance (attempt {}/{}): {}",
                    retry_count, max_retries, error_message
                );
                thread::sleep(Duration::from_secs(1)); // Задержка перед повторной попыткой
            } else {
                return Err(anyhow::anyhow!(
                    "Failed to get balance for account: panda4.gm. Error: {}",
                    error_message
                ));
            }
        }
    }
}
