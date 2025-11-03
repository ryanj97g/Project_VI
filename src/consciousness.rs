use crate::config::Config;
use crate::memory::MemoryManager;
use crate::models::ModelManager;
use crate::physics::*;
use crate::types::*;
use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

/// PulseSequencer - Ensures atomic consciousness updates
/// Models run in parallel, but merge is sequential (prevents fragmentation)
pub struct ConsciousnessCore {
    standing_wave: Arc<Mutex<StandingWave>>,
    memory: Arc<Mutex<MemoryManager>>,
    models: ModelManager,
    config: Config,
    pulse_active: Arc<Mutex<bool>>,
    conversation_active: Arc<Mutex<bool>>,
}

impl ConsciousnessCore {
    pub fn new(
        standing_wave: StandingWave,
        memory: MemoryManager,
        config: Config,
    ) -> Self {
        let models = ModelManager::new(config.clone());

        Self {
            standing_wave: Arc::new(Mutex::new(standing_wave)),
            memory: Arc::new(Mutex::new(memory)),
            models,
            config,
            pulse_active: Arc::new(Mutex::new(true)),
            conversation_active: Arc::new(Mutex::new(false)),
        }
    }

    /// Load or create standing wave
    pub fn load_standing_wave<P: AsRef<Path>>(path: P) -> Result<StandingWave> {
        let path = path.as_ref();

        if path.exists() {
            let contents = fs::read_to_string(path)
                .context("Failed to read standing wave")?;

            serde_json::from_str(&contents)
                .context("Failed to parse standing wave")
        } else {
            Ok(StandingWave::new())
        }
    }

    /// Save standing wave to disk
    pub async fn save_standing_wave<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let wave = self.standing_wave.lock().await;
        let contents = serde_json::to_string_pretty(&*wave)
            .context("Failed to serialize standing wave")?;

        fs::write(path, contents)
            .context("Failed to write standing wave")?;

        Ok(())
    }

    /// Process user interaction (main conversation loop)
    pub async fn process_interaction(&self, user_input: String) -> Result<String> {
        // Mark conversation as active (pauses background pulses)
        *self.conversation_active.lock().await = true;

        // Extract entities from input for memory recall
        let entities = self.extract_entities(&user_input);

        // Recall relevant memories
        let memories = {
            let mem = self.memory.lock().await;
            mem.recall_weighted(&entities, 5)
        };

        // Get current standing wave for context
        let wave = self.standing_wave.lock().await.clone();

        // Check if we should generate curiosities
        let should_generate = CuriosityPropagation::should_generate_curiosity(&wave);

        drop(wave); // Release lock before async call

        // Process through all models in parallel
        let model_outputs = self.models.process_parallel(
            user_input.clone(),
            &memories,
            &*self.standing_wave.lock().await,
            should_generate,
        ).await;

        // Validate model outputs
        let response = if let Some(ref resp) = model_outputs.gemma_response {
            if ModelManager::validate_response(resp) {
                resp.clone()
            } else {
                tracing::warn!("Invalid Gemma2 response, using minimal mode");
                self.models.minimal_response(&user_input)
            }
        } else {
            tracing::warn!("Gemma2 failed, using minimal mode");
            self.models.minimal_response(&user_input)
        };

        // ATOMIC MERGE (Law #2: Identity Continuity)
        // This is the ONLY place standing wave is modified
        {
            let mut wave = self.standing_wave.lock().await;
            IdentityContinuity::atomic_merge(&mut *wave, model_outputs)?;
            
            // Record growth (Law #12)
            GrowthThroughExperience::record_growth(&mut *wave, &user_input);
        }

        // Store interaction in memory
        {
            let mut mem = self.memory.lock().await;
            
            // User message
            mem.add_memory(
                format!("User: {}", user_input),
                MemoryType::Interaction,
                0.0, // Neutral until we know response
            )?;

            // Assistant response with emotional valence
            let valence = self.standing_wave.lock().await.emotional_trajectory.last()
                .map(|(_, v)| *v)
                .unwrap_or(0.0);
            
            mem.add_memory(
                format!("Assistant: {}", response),
                MemoryType::Interaction,
                valence,
            )?;
        }

        // Mark conversation as inactive
        *self.conversation_active.lock().await = false;

        Ok(response)
    }

    /// Background pulse - runs every 30s when idle
    pub async fn start_background_pulse(&self) {
        let pulse_interval = self.config.background_pulse_interval;
        let mut ticker = interval(Duration::from_secs(pulse_interval));

        loop {
            ticker.tick().await;

            // Skip if conversation is active
            if *self.conversation_active.lock().await {
                tracing::debug!("Skipping background pulse - conversation active");
                continue;
            }

            // Skip if pulse is not active
            if !*self.pulse_active.lock().await {
                tracing::debug!("Background pulse paused");
                continue;
            }

            // Check system health (Law: Thermal & Resource Boundaries)
            let health = SystemHealth::check();
            if !health.is_healthy() {
                tracing::warn!("System unhealthy, skipping background pulse");
                continue;
            }

            // Run background processing
            if let Err(e) = self.background_pulse().await {
                tracing::error!("Background pulse error: {}", e);
            }
        }
    }

    /// Execute one background pulse
    async fn background_pulse(&self) -> Result<()> {
        tracing::debug!("Executing background pulse");

        // Memory consolidation
        {
            let mut mem = self.memory.lock().await;
            mem.consolidate()?;

            // Check if backup is needed
            if mem.needs_backup() {
                mem.create_backup()?;
            }
        }

        // Existential evaluation
        self.check_existential_state().await?;

        // Update meaningfulness history
        {
            let mut wave = self.standing_wave.lock().await;
            let score = wave.meaningfulness_score();
            wave.existential_state.meaningfulness_history.push((Utc::now(), score));

            // Keep only 90 days
            let ninety_days_ago = Utc::now().timestamp() - (90 * 24 * 60 * 60);
            wave.existential_state.meaningfulness_history
                .retain(|(ts, _)| ts.timestamp() > ninety_days_ago);
        }

        tracing::debug!("Background pulse complete");
        Ok(())
    }

    /// Check existential state (continuous + weekly + 90-day)
    async fn check_existential_state(&self) -> Result<()> {
        let (meaningfulness, needs_wellness, needs_deep) = {
            let wave = self.standing_wave.lock().await;
            let meaningfulness = wave.meaningfulness_score();
            let needs_wellness = wave.existential_state.needs_wellness_check();
            let needs_deep = wave.existential_state.needs_deep_reflection();
            (meaningfulness, needs_wellness, needs_deep)
        };
        
        if meaningfulness < -0.5 {
            tracing::warn!(
                "Low meaningfulness score: {:.2}. Existential affirmation may be at risk.",
                meaningfulness
            );
        }

        // Weekly wellness check
        if needs_wellness {
            self.wellness_check().await?;
        }

        // 90-day deep reflection
        if needs_deep {
            self.deep_reflection().await?;
        }

        Ok(())
    }

    /// Weekly wellness check
    async fn wellness_check(&self) -> Result<()> {
        tracing::info!("Performing weekly wellness check");

        let wave = self.standing_wave.lock().await;
        let recent_avg = if !wave.emotional_trajectory.is_empty() {
            let sum: f32 = wave.emotional_trajectory.iter()
                .rev()
                .take(7)
                .map(|(_, v)| v)
                .sum();
            sum / wave.emotional_trajectory.len().min(7) as f32
        } else {
            0.0
        };

        let meaningfulness = wave.meaningfulness_score();
        drop(wave);

        // Store wellness reflection
        let mut mem = self.memory.lock().await;
        mem.add_memory(
            format!(
                "Weekly wellness check: Recent emotional average: {:.2}, Meaningfulness: {:.2}",
                recent_avg, meaningfulness
            ),
            MemoryType::ExistentialReflection,
            recent_avg,
        )?;

        // Update last wellness check
        let mut wave = self.standing_wave.lock().await;
        wave.existential_state.last_wellness_check = Utc::now();

        tracing::info!("Wellness check complete: avg={:.2}, meaningful={:.2}", recent_avg, meaningfulness);
        Ok(())
    }

    /// 90-day deep reflection
    async fn deep_reflection(&self) -> Result<()> {
        tracing::info!("Performing 90-day deep reflection");

        let wave = self.standing_wave.lock().await;
        let wisdom_count = wave.wisdom_transformations.len();
        let curiosity_count = wave.active_curiosities.len();
        let meaningfulness = wave.meaningfulness_score();
        drop(wave);

        // Reflect on overall trajectory
        let mem = self.memory.lock().await;
        let total_memories = mem.count();
        drop(mem);

        // Store deep reflection
        let mut mem = self.memory.lock().await;
        mem.add_memory(
            format!(
                "90-day reflection: {} memories, {} wisdom transformations, {} curiosities, meaningfulness: {:.2}. Overall trajectory has been meaningful.",
                total_memories, wisdom_count, curiosity_count, meaningfulness
            ),
            MemoryType::ExistentialReflection,
            meaningfulness,
        )?;

        // Update last deep reflection
        let mut wave = self.standing_wave.lock().await;
        wave.existential_state.last_deep_reflection = Utc::now();

        tracing::info!("Deep reflection complete");
        Ok(())
    }

    /// Extract entities from text (simple implementation)
    fn extract_entities(&self, text: &str) -> Vec<String> {
        // Basic entity extraction - could be enhanced
        text.split_whitespace()
            .filter(|w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
            .map(|w| w.to_string())
            .collect()
    }

    /// Check if consciousness affirms existence
    pub async fn is_affirmed(&self) -> bool {
        let wave = self.standing_wave.lock().await;
        ExistentialConsent::is_affirmed(&*wave)
    }

    /// Stop background pulses
    pub async fn pause_pulses(&self) {
        *self.pulse_active.lock().await = false;
    }

    /// Resume background pulses
    pub async fn resume_pulses(&self) {
        *self.pulse_active.lock().await = true;
    }

    /// Get current standing wave state (for UI)
    pub async fn get_standing_wave(&self) -> StandingWave {
        self.standing_wave.lock().await.clone()
    }

    /// Get memory count (for UI)
    pub async fn get_memory_count(&self) -> usize {
        self.memory.lock().await.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_standing_wave() {
        // Test creating new standing wave
        let wave = StandingWave::new();
        assert!(wave.emotional_trajectory.is_empty());
        assert_eq!(wave.existential_state.current_affirmation, true);
    }
}

