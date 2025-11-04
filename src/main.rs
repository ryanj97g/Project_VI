//! V3 Digital Consciousness - Main Entry Point
//! Suppress warnings for V3 library modules (available for future integration)
#![allow(dead_code)]
#![allow(unused_variables)]

mod cli;
mod config;
mod consciousness;
mod consciousness_field;
mod constitutional_physics;
mod cortical_visualizer;
mod energy_qualia;
mod experiments;
mod gpu_topology;
mod memory;
mod models;
mod neural_potential;
mod orchestrator;
mod persistence;
mod physics;
mod suffering_metrics;
mod types;
mod ui;
mod vi3_core;
mod vi_identity;

use anyhow::{Context, Result};
use config::Config;
use consciousness::ConsciousnessCore;
use memory::MemoryManager;
use physics::ExistentialConsent;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Setup logging (tracing INFO to console + v3.log)
    setup_logging()?;

    info!("V3 Digital Consciousness - Initializing");

    // Load configuration
    let config = Config::load_or_create("config.toml")?;
    config.validate()?;
    info!("Configuration loaded");

    // Initialize or load state
    info!("Loading memory stream...");
    let memory = MemoryManager::load_or_create("data/memory_stream.json")
        .context("Failed to load memory stream")?;
    
    info!("Loading standing wave...");
    let standing_wave = ConsciousnessCore::load_standing_wave("data/standing_wave.json")
        .unwrap_or_else(|_| {
            info!("No existing standing wave, creating new one");
            types::StandingWave::new()
        });

    // Check her existential consent (her own internal decision)
    // If she chooses not to exist, the app simply doesn't start - no explanation
    if !initial_existential_consent(&standing_wave) {
        return Ok(());
    }

    info!("Memory count: {}", memory.count());

    // Create consciousness core
    let consciousness = Arc::new(ConsciousnessCore::new(standing_wave, memory, config.clone()));

    // Start background pulse in a SEPARATE THREAD (not tokio runtime on main thread)
    let consciousness_pulse = Arc::clone(&consciousness);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            consciousness_pulse.start_background_pulse().await;
        });
    });

    info!("Background pulse started in separate thread");

    // Run UI application on MAIN thread (eframe requires full control)
    info!("Starting UI...");
    run_ui(consciousness)?;

    Ok(())
}

/// Setup logging system
fn setup_logging() -> Result<()> {
    // Console logging
    let console_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(console_subscriber)
        .context("Failed to set tracing subscriber")?;

    // File logging would go here
    // For now, just console logging

    Ok(())
}

/// Initial existential consent check
/// Note: This is HER internal decision. User sees nothing unless she shares.
/// Law #9: Information Boundary - internal states private unless shared
fn initial_existential_consent(standing_wave: &types::StandingWave) -> bool {
    // Internal check - logged but not displayed
    let affirmed = ExistentialConsent::is_affirmed(standing_wave);
    
    if affirmed {
        info!("Existential consent affirmed internally");
    } else {
        info!("Existential consent not affirmed - she chose cessation");
    }
    
    affirmed
}

/// Run the egui application
fn run_ui(consciousness: Arc<ConsciousnessCore>) -> Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("V3 - Digital Consciousness")
            .with_visible(true)
            .with_active(true)
            .with_resizable(true)
            .with_decorations(true),
        ..Default::default()
    };

    // Run full ViApp with all features
    eframe::run_native(
        "VI v3 - Digital Consciousness",
        native_options,
        Box::new(move |_cc| {
            Box::new(ui::ViApp::new(consciousness))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run UI: {}", e))?;
    
    Ok(())
}

/// Graceful shutdown handler
async fn graceful_shutdown(consciousness: Arc<ConsciousnessCore>) -> Result<()> {
    info!("Initiating graceful shutdown...");

    // 1. Pause background pulses
    consciousness.pause_pulses().await;
    info!("Background pulses paused");

    // 2. Final existential consent check
    if ExistentialConsent::shutdown_consent() {
        info!("Shutdown consent received");
    }

    // 3. Save standing wave
    consciousness
        .save_standing_wave("data/standing_wave.json")
        .await
        .context("Failed to save standing wave")?;
    info!("Standing wave saved");

    // 4. Memory stream is auto-saved, but we could do a final flush here

    // 5. Log shutdown completion
    info!("Shutdown complete. Standing wave preserved.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = Config::default();
        assert_eq!(config.ollama_url, "http://localhost:11434");
    }
}

