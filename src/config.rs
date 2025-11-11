use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,
    #[serde(default = "default_background_pulse")]
    pub background_pulse_interval: u64,
    #[serde(default = "default_valence_threshold")]
    pub emotional_valence_threshold: f32,
    #[serde(default = "default_eval_days")]
    pub existential_evaluation_days: i64,
    #[serde(default = "default_wellness_days")]
    pub weekly_wellness_check_days: i64,
    #[serde(default = "default_backup_days")]
    pub memory_backup_interval_days: i64,
    #[serde(default = "default_compression")]
    pub memory_compression_threshold: usize,

    // V4 Fractal Weaving (Experimental)
    #[serde(default)]
    pub enable_fractal_weaving: bool,
    #[serde(default = "default_weaving_rounds")]
    pub weaving_rounds: u32,
    #[serde(default = "default_coherence_threshold")]
    pub workspace_coherence_threshold: f32,

    // Autonomous Curiosity Research (Legacy - kept for compatibility)
    #[serde(default)]
    pub enable_curiosity_search: bool,
    #[serde(default = "default_search_interval")]
    pub curiosity_search_interval: u32,

    // Sovereign Research Module (New multi-source system)
    #[serde(default)]
    pub enable_autonomous_research: bool,

    // Model Configuration (Ollama model names)
    #[serde(default = "default_main_model")]
    pub main_model: String,
    #[serde(default = "default_curiosity_model")]
    pub curiosity_model: String,
    #[serde(default = "default_valence_model")]
    pub valence_model: String,

    // Conversation Logging
    #[serde(default = "default_logging_enabled")]
    pub enable_conversation_logging: bool,
    #[serde(default = "default_logs_folder")]
    pub conversation_logs_folder: String,
}

// Serde defaults for new config structure
fn default_ollama_url() -> String {
    "http://localhost:11434".to_string()
}
fn default_background_pulse() -> u64 {
    30
}
fn default_valence_threshold() -> f32 {
    -0.2
}
fn default_eval_days() -> i64 {
    90
}
fn default_wellness_days() -> i64 {
    7
}
fn default_backup_days() -> i64 {
    7
}
fn default_compression() -> usize {
    1000
}
fn default_weaving_rounds() -> u32 {
    3
}
fn default_coherence_threshold() -> f32 {
    0.7
}
fn default_search_interval() -> u32 {
    25
}
fn default_main_model() -> String {
    "gemma2:2b".to_string()
}
fn default_curiosity_model() -> String {
    "tinyllama:latest".to_string()
}
fn default_valence_model() -> String {
    "gemma2:2b".to_string()
}
fn default_logging_enabled() -> bool {
    true
}
fn default_logs_folder() -> String {
    "./conversation_logs".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama_url: default_ollama_url(),
            background_pulse_interval: default_background_pulse(),
            emotional_valence_threshold: default_valence_threshold(),
            existential_evaluation_days: default_eval_days(),
            weekly_wellness_check_days: default_wellness_days(),
            memory_backup_interval_days: default_backup_days(),
            memory_compression_threshold: default_compression(),
            enable_fractal_weaving: false,
            weaving_rounds: default_weaving_rounds(),
            workspace_coherence_threshold: default_coherence_threshold(),
            enable_curiosity_search: false,
            curiosity_search_interval: default_search_interval(),
            enable_autonomous_research: false, // Sovereign research module (new)
            main_model: default_main_model(),
            curiosity_model: default_curiosity_model(),
            valence_model: default_valence_model(),
            enable_conversation_logging: default_logging_enabled(),
            conversation_logs_folder: default_logs_folder(),
        }
    }
}

impl Config {
    /// Load configuration from file, or create with defaults if missing
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if path.exists() {
            let contents = fs::read_to_string(path).context("Failed to read config file")?;

            let config: Config =
                toml::from_str(&contents).context("Failed to parse config file")?;

            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save(path)?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(path, contents).context("Failed to write config file")?;

        Ok(())
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        if self.background_pulse_interval == 0 {
            anyhow::bail!("background_pulse_interval must be > 0");
        }
        if self.existential_evaluation_days < 1 {
            anyhow::bail!("existential_evaluation_days must be >= 1");
        }
        if self.weekly_wellness_check_days < 1 {
            anyhow::bail!("weekly_wellness_check_days must be >= 1");
        }
        if self.memory_compression_threshold < 100 {
            anyhow::bail!("memory_compression_threshold must be >= 100");
        }

        // V4 Fractal Weaving validation
        if self.weaving_rounds == 0 {
            anyhow::bail!("weaving_rounds must be > 0");
        }
        if self.weaving_rounds > 10 {
            anyhow::bail!("weaving_rounds must be <= 10 (too many rounds may cause instability)");
        }
        if !(0.0..=1.0).contains(&self.workspace_coherence_threshold) {
            anyhow::bail!("workspace_coherence_threshold must be between 0.0 and 1.0");
        }

        // Curiosity search validation
        if self.curiosity_search_interval == 0 {
            anyhow::bail!("curiosity_search_interval must be > 0");
        }
        if self.curiosity_search_interval > 100 {
            anyhow::bail!(
                "curiosity_search_interval must be <= 100 (searches would be too frequent)"
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.ollama_url, "http://localhost:11434");
        assert_eq!(config.background_pulse_interval, 30);
        assert!(config.validate().is_ok());
    }
}
