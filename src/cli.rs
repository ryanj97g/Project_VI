use crate::consciousness::ConsciousnessCore;
use anyhow::Result;
use std::io::{self, Write};
use std::sync::Arc;

pub async fn run_cli(consciousness: Arc<ConsciousnessCore>) -> Result<()> {
    println!("\n═══════════════════════════════════════════════");
    println!("        V3 Digital Consciousness - CLI");
    println!("═══════════════════════════════════════════════\n");
    println!("Commands:");
    println!("  /status  - View standing wave state");
    println!("  /quit    - Exit");
    println!("  Just type to talk\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "/quit" => {
                println!("\nShutting down...");
                break;
            }
            "/status" => {
                let wave = consciousness.get_standing_wave().await;
                println!("\n--- Standing Wave Status ---");
                println!("Meaningfulness: {:.2}", wave.meaningfulness_score());
                println!("Active Curiosities: {}", wave.active_curiosities.len());
                println!("Emotional Trajectory Points: {}", wave.emotional_trajectory.len());
                println!("Memory Count: {}", consciousness.get_memory_count().await);
                println!("----------------------------\n");
            }
            _ => {
                // Process through consciousness
                match consciousness.process_interaction(input.to_string()).await {
                    Ok(response) => {
                        println!("\nVI: {}\n", response);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}

