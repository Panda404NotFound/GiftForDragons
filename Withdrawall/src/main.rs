use std::time::Duration;
use tokio;
use std::process::Command;
use anyhow::Result;
use regex::Regex;

async fn prepare_deposit(symbols: &[(&str, &str)]) -> Result<(), anyhow::Error> {
    println!("Start prepare_deposit...");
    let symbols_arg = symbols
        .iter()
        .map(|(sym, contract)| format!("{{\"sym\": \"{}\", \"contract\": \"{}\"}}", sym, contract))
        .collect::<Vec<_>>()
        .join(", ");

        let output = Command::new("sh")
        .arg("-c")
        .arg(format!("proton action proton.swaps depositprep '{{\"owner\": \"panda4.gm\", \"symbols\": [{}]}}' panda4.gm", symbols_arg))
        .output()?;

    if output.status.success() {
        println!("Prepare_deposit output: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        eprintln!(
            "Failed to prepare deposit. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err(anyhow::anyhow!("Failed to prepare deposit"))
    }
}

async fn withdraw_all() -> Result<(), anyhow::Error> {
    println!("p_c.rs: Start withdraw_all...");

    let error_regex = Regex::new(r"AbortError: The user aborted a request|Please create record at deposit table first with depositprep").unwrap();
    let mut attempts = 0;

    loop {
        let output = Command::new("sh")
        .arg("-c")
        .arg(r#"proton action proton.swaps withdrawall '{"owner":"panda4.gm"}' panda4.gm"#)
        .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        if output.status.success() && !error_regex.is_match(&output_str) {
            println!("Withdraw all successful. Output: {}", output_str);
            return Ok(());
        } else {
            attempts += 1;
            eprintln!("Attempt {}: Failed to withdraw all. Error: {}", attempts, output_str);

            if attempts >= 3 {
                return Err(anyhow::anyhow!("Failed to withdraw all after 3 attempts"));
            } else {
                tokio::time::sleep(Duration::from_secs(2)).await; // Wait for 2 seconds before the next attempt
            }
        }
    }
}

#[tokio::main]
async fn main() {
    loop {
        match run_main_loop().await {
            Ok(_) => {
                println!("Main loop completed successfully.");
            }
            Err(e) => {
                eprintln!("Error in main loop: {}", e);
            }
        }
        // Ожидание 300 секунд перед следующим запуском run_main_loop
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}

async fn run_main_loop() -> Result<(), anyhow::Error> {
    println!("Proton.swaps withdrawall");

    // Подготовка депозита для XUSDC и SNIPS
    let xusdc_symbols = &[("6,XUSDC", "xtokens"), ("4,XPR", "eosio.token")];
    let snips_symbols = &[("4,SNIPS", "snipcoins"), ("4,XPR", "eosio.token")];

    let (xusdc_result, snips_result) = tokio::try_join!(
        tokio::spawn(async move {
            println!("p_c.rs: Preparing deposit for XUSDC...");
            prepare_deposit(xusdc_symbols).await
        }),
        tokio::spawn(async move {
            println!("p_c.rs: Preparing deposit for SNIPS...");
            prepare_deposit(snips_symbols).await
        })
    )?;

    xusdc_result?;
    snips_result?;

    match withdraw_all().await {
        Ok(_) => println!("Withdraw all successful."),
        Err(e) => {
            eprintln!("Error in withdraw_all: {}", e);
            return Err(e);
        }
    }

    Ok(())
}