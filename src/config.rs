use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ollama_url: String,
    pub background_pulse_interval: u64,
    pub emotional_valence_threshold: f32,
    pub existential_evaluation_days: i64,
    pub weekly_wellness_check_days: i64,
    pub memory_backup_interval_days: i64,
    pub memory_compression_threshold: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            background_pulse_interval: 30,
            emotional_valence_threshold: -0.2,
            existential_evaluation_days: 90,
            weekly_wellness_check_days: 7,
            memory_backup_interval_days: 7,
            memory_compression_threshold: 1000,
        }
    }
}

impl Config {
    /// Load configuration from file, or create with defaults if missing
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if path.exists() {
            let contents = fs::read_to_string(path)
                .context("Failed to read config file")?;
            
            let config: Config = toml::from_str(&contents)
                .context("Failed to parse config file")?;
            
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
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(path, contents)
            .context("Failed to write config file")?;
        
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

