use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;
use tokio::task;

//use std::process::Command;
//use std::thread;

async fn claim_rewards(stake: &str, pool: &str) -> Result<(), anyhow::Error> {
    let command = format!(
        r#"proton action yield.farms claim '{{"claimer": "panda4.gm", "stakes": ["{}"]}}' panda4.gm"#,
        stake
    );

    let output = AsyncCommand::new("bash")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdout = output.stdout.expect("Failed to get stdout");
    let stderr = output.stderr.expect("Failed to get stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    while let Some(line) = stdout_reader.next_line().await.unwrap_or(None) {
        println!("Claimed rewards for stake: {} in pool: {}. Output: {}", stake, pool, line);
    }

    while let Some(line) = stderr_reader.next_line().await.unwrap_or(None) {
        eprintln!("Error claiming rewards for stake: {} in pool: {}. Error: {}", stake, pool, line);
    }

    Ok(())
}

async fn claim_rewards_xpr(i: usize) {
    loop {
        println!("Claiming rewards for XPR in thread {}...", i);
        if let Err(e) = claim_rewards("XPRUSDC", "XPR").await {
            eprintln!("Error in claim_rewards for XPR (thread {}): {}", i, e);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

async fn claim_rewards_snips(i: usize) {
    loop {
        println!("Claiming rewards for SNIPS in thread {}...", i);
        if let Err(e) = claim_rewards("SNIPSXP", "SNIPS").await {
            eprintln!("Error in claim_rewards for SNIPS (thread {}): {}", i, e);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

// Основная функция которая отвечает за нагрузку процессора 
// num_instances - это копии основного цикла 
// num_threads_per_instance - это потоки асинхронных операций 
// нужно проверить сколько жрет и оптимальное число копий и потоков
#[tokio::main]
async fn main() {
    let num_instances = 2; // Количество копий 
    let num_threads_per_instance = 2; // Количество потоков 

    let mut instances = Vec::new();
    for i in 0..num_instances {
        let instance = tokio::spawn(async move {
            println!("Starting instance {}", i);
            let mut threads = Vec::new();

            for j in 0..num_threads_per_instance {
                let xpr_thread = task::spawn(claim_rewards_xpr(j));
                let snips_thread = task::spawn(claim_rewards_snips(j));
                threads.push(xpr_thread);
                threads.push(snips_thread);
            }

            for thread in threads {
                let _ = thread.await;
            }
        });
        instances.push(instance);
    }

    for instance in instances {
        instance.await.expect("Failed to join instance");
    }
}


// Без задержки ожидания выполнения 
// только с счетчиком 
// может жрать много процессов.

/*
#[tokio::main]
async fn main() {
    let num_instances = 3;
    let num_threads_per_instance = 2;

    let mut instances = Vec::new();
    for i in 0..num_instances {
        let instance = tokio::spawn(async move {
            println!("Starting instance {}", i);
            let mut threads = Vec::new();

            for j in 0..num_threads_per_instance {
                let xpr_thread = task::spawn(claim_rewards_xpr(j));
                let snips_thread = task::spawn(claim_rewards_snips(j));
                threads.push(xpr_thread);
                threads.push(snips_thread);
            }

            for thread in threads {
                thread.await;
            }
        });
        instances.push(instance);
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        for i in 0..num_instances {
            let instance = tokio::spawn(async move {
                println!("Starting instance {}", i);
                let mut threads = Vec::new();

                for j in 0..num_threads_per_instance {
                    let xpr_thread = task::spawn(claim_rewards_xpr(j));
                    let snips_thread = task::spawn(claim_rewards_snips(j));
                    threads.push(xpr_thread);
                    threads.push(snips_thread);
                }

                for thread in threads {
                    thread.await;
                }
            });
            instances.push(instance);
        }
    }
}
*/
