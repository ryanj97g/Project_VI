/// Ollama Performance Monitor
/// Polls Ollama API for system metrics and model performance

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaStatus {
    pub online: bool,
    pub active_models: Vec<RunningModel>,
    pub system_resources: SystemResources,
}

impl OllamaStatus {
    pub fn offline() -> Self {
        Self {
            online: false,
            active_models: Vec::new(),
            system_resources: SystemResources::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningModel {
    pub name: String,
    pub size_gb: f32,
    pub processor: f32,        // Utilization percentage
    pub tokens_per_sec: f32,
    pub vram_used_gb: f32,
    pub vram_total_gb: f32,
    pub context_used: usize,
    pub context_total: usize,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemResources {
    pub gpu_util_percent: f32,
    pub total_vram_used_gb: f32,
    pub total_vram_total_gb: f32,
    pub system_ram_used_gb: f32,
    pub system_ram_total_gb: f32,
    pub active_model_count: usize,
}

/// Simple performance history tracker
#[derive(Debug, Clone)]
pub struct PerformanceHistory {
    pub tokens_per_sec: Vec<f32>,
    pub gpu_util: Vec<f32>,
    max_samples: usize,
}

impl PerformanceHistory {
    pub fn new(max_samples: usize) -> Self {
        Self {
            tokens_per_sec: Vec::new(),
            gpu_util: Vec::new(),
            max_samples,
        }
    }

    pub fn add_sample(&mut self, tokens: f32, gpu: f32) {
        self.tokens_per_sec.push(tokens);
        self.gpu_util.push(gpu);

        // Keep only last N samples
        if self.tokens_per_sec.len() > self.max_samples {
            self.tokens_per_sec.remove(0);
        }
        if self.gpu_util.len() > self.max_samples {
            self.gpu_util.remove(0);
        }
    }
}

/// Ollama monitor client
pub struct OllamaMonitor {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaMonitor {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { base_url, client }
    }

    /// Poll Ollama and get current status
    pub async fn get_status(&self) -> OllamaStatus {
        // Try to get running models from /api/ps
        let active_models = match self.get_running_models().await {
            Ok(models) => models,
            Err(_) => {
                // Ollama is offline or unreachable
                return OllamaStatus::offline();
            }
        };

        // Get system resources
        let system_resources = self.get_system_resources(&active_models).await;

        OllamaStatus {
            online: true,
            active_models,
            system_resources,
        }
    }

    async fn get_running_models(&self) -> Result<Vec<RunningModel>> {
        let url = format!("{}/api/ps", self.base_url);
        let response: serde_json::Value = self.client.get(&url).send().await?.json().await?;

        let mut models = Vec::new();

        if let Some(model_array) = response["models"].as_array() {
            for model in model_array {
                let name = model["name"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string();
                let size_vram = model["size_vram"]
                    .as_u64()
                    .unwrap_or(0) as f32
                    / 1_073_741_824.0; // Convert bytes to GB

                let processor = model["processor"].as_str().unwrap_or("100");
                let processor_percent = processor
                    .trim_end_matches('%')
                    .parse::<f32>()
                    .unwrap_or(100.0);

                // Estimate tokens/sec and other metrics from available data
                let expires_at = model["expires_at"].as_str().unwrap_or("");
                let uptime_secs = if !expires_at.is_empty() {
                    // Estimate uptime (Ollama expires models after inactivity)
                    60 // Placeholder
                } else {
                    0
                };

                models.push(RunningModel {
                    name: name.clone(),
                    size_gb: size_vram,
                    processor: processor_percent,
                    tokens_per_sec: 20.0 + (100.0 - processor_percent) / 5.0, // Estimate
                    vram_used_gb: size_vram,
                    vram_total_gb: size_vram * 1.2, // Estimate
                    context_used: 512, // Placeholder - not available in API
                    context_total: 4096, // Placeholder
                    uptime_secs,
                });
            }
        }

        Ok(models)
    }

    async fn get_system_resources(&self, active_models: &[RunningModel]) -> SystemResources {
        let mut total_vram_used = 0.0;
        let mut total_gpu_util = 0.0;

        for model in active_models {
            total_vram_used += model.vram_used_gb;
            total_gpu_util += model.processor;
        }

        // Average GPU utilization
        let gpu_util_percent = if !active_models.is_empty() {
            total_gpu_util / active_models.len() as f32
        } else {
            0.0
        };

        // Estimate system RAM (not available in Ollama API)
        let (ram_used, ram_total) = Self::estimate_system_ram();

        SystemResources {
            gpu_util_percent,
            total_vram_used_gb: total_vram_used,
            total_vram_total_gb: 8.0, // Default - user should check with nvidia-smi
            system_ram_used_gb: ram_used,
            system_ram_total_gb: ram_total,
            active_model_count: active_models.len(),
        }
    }

    fn estimate_system_ram() -> (f32, f32) {
        // Use sysinfo crate (already a dependency in Cargo.toml)
        use sysinfo::System;
        let mut sys = System::new_all();
        sys.refresh_memory();
        let used = sys.used_memory() as f32 / 1_073_741_824.0;
        let total = sys.total_memory() as f32 / 1_073_741_824.0;
        (used, total)
    }
}

