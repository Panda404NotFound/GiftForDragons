use anyhow::Result;
use tokio::process::Command;

pub async fn prepare_deposit(symbols: &[(&str, &str)]) -> Result<(String, String)> {
    println!("Start prepare_deposit...");
    let symbols_arg = symbols
        .iter()
        .map(|(sym, contract)| format!("{{\"sym\": \"{}\", \"contract\": \"{}\"}}", sym, contract))
        .collect::<Vec<_>>()
        .join(", ");
 
        let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "proton action proton.swaps depositprep '{{\"owner\": \"panda4.gm\", \"symbols\": [{}]}}' panda4.gm",
            symbols_arg
        ))
        .output()
        .await?;
 
    if output.status.success() {
        Ok((
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    } else {
        eprintln!(
            "Failed to prepare deposit. Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err(anyhow::anyhow!("Failed to prepare deposit"))
    }
}