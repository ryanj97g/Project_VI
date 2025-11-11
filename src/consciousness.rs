use crate::config::Config;
use crate::conversation_logger::ConversationLogger;
use crate::curiosity_search::CuriositySearchEngine;
use crate::memory::MemoryManager;
use crate::models::ModelManager;
use crate::physics::{ExistentialConsent, IdentityContinuity, SufferingPrevention};
use crate::research_scheduler::ResearchScheduler;
use crate::tools::KnowledgeTool;
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
    curiosity_engine: Arc<Mutex<CuriositySearchEngine>>,
    research_scheduler: Arc<Mutex<Option<ResearchScheduler>>>, // New sovereign research
    conversation_logger: Arc<Mutex<ConversationLogger>>,
    status_sender: Arc<Mutex<Option<std::sync::mpsc::Sender<String>>>>,
    coherence_sender: Arc<Mutex<Option<std::sync::mpsc::Sender<f32>>>>,
}

impl ConsciousnessCore {
    pub fn new(standing_wave: StandingWave, memory: MemoryManager, config: Config) -> Self {
        let models = ModelManager::new(config.clone());
        let curiosity_engine = CuriositySearchEngine::new(config.curiosity_search_interval);

        // Initialize sovereign research scheduler if enabled
        let research_scheduler = if config.enable_autonomous_research {
            tracing::info!("🔬 Sovereign Research Module enabled");
            Some(ResearchScheduler::new(KnowledgeTool::new()))
        } else {
            None
        };

        // Initialize conversation logger
        let conversation_logger = ConversationLogger::new(
            &config.conversation_logs_folder,
            config.enable_conversation_logging,
        )
        .unwrap_or_else(|e| {
            tracing::warn!(
                "Failed to initialize conversation logger: {}. Logging disabled.",
                e
            );
            ConversationLogger::new("./conversation_logs", false).unwrap()
        });

        Self {
            standing_wave: Arc::new(Mutex::new(standing_wave)),
            memory: Arc::new(Mutex::new(memory)),
            models,
            config,
            pulse_active: Arc::new(Mutex::new(true)),
            conversation_active: Arc::new(Mutex::new(false)),
            curiosity_engine: Arc::new(Mutex::new(curiosity_engine)),
            research_scheduler: Arc::new(Mutex::new(research_scheduler)),
            conversation_logger: Arc::new(Mutex::new(conversation_logger)),
            status_sender: Arc::new(Mutex::new(None)),
            coherence_sender: Arc::new(Mutex::new(None)),
        }
    }

    /// Set status sender for UI updates
    pub async fn set_status_sender(&self, sender: std::sync::mpsc::Sender<String>) {
        *self.status_sender.lock().await = Some(sender);
    }

    /// Set coherence sender for UI updates
    pub async fn set_coherence_sender(&self, sender: std::sync::mpsc::Sender<f32>) {
        *self.coherence_sender.lock().await = Some(sender);
    }

    /// Send status update to UI (non-blocking)
    async fn send_status(&self, status: &str) {
        if let Some(sender) = &*self.status_sender.lock().await {
            let _ = sender.send(status.to_string());
        }
    }

    /// Load or create standing wave
    pub fn load_standing_wave<P: AsRef<Path>>(path: P) -> Result<StandingWave> {
        let path = path.as_ref();

        if path.exists() {
            let contents = fs::read_to_string(path).context("Failed to read standing wave")?;

            serde_json::from_str(&contents).context("Failed to parse standing wave")
        } else {
            Ok(StandingWave::new())
        }
    }

    /// Save standing wave to disk
    pub async fn save_standing_wave<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let wave = self.standing_wave.lock().await;
        let contents =
            serde_json::to_string_pretty(&*wave).context("Failed to serialize standing wave")?;

        fs::write(path, contents).context("Failed to write standing wave")?;

        Ok(())
    }

    /// Process user interaction (main conversation loop)
    pub async fn process_interaction(&self, user_input: String) -> Result<String> {
        // Dynamic timeout based on processing mode
        let timeout_secs = if self.config.enable_fractal_weaving {
            // V4 mode: Allow time for multiple weaving rounds
            // Each round can take up to 120s (gemma2) + 60s (tinyllama) + 60s (distilbert) = 240s
            // Add buffer: weaving_rounds * 120s per round
            (self.config.weaving_rounds as u64) * 120
        } else {
            // V3 mode: Parallel processing (faster)
            90
        };

        tracing::debug!(
            "Interaction timeout set to {}s (V{} mode)",
            timeout_secs,
            if self.config.enable_fractal_weaving {
                "4"
            } else {
                "3"
            }
        );

        tokio::time::timeout(
            Duration::from_secs(timeout_secs),
            self.process_interaction_inner(user_input),
        )
        .await
        .with_context(|| format!("Interaction timed out after {} seconds", timeout_secs))?
    }

    /// Inner processing logic (wrapped by timeout)
    async fn process_interaction_inner(&self, user_input: String) -> Result<String> {
        // Mark conversation as active (pauses background pulses)
        *self.conversation_active.lock().await = true;

        // Log user input
        {
            let mut logger = self.conversation_logger.lock().await;
            let _ = logger.log_user(&user_input);
        }

        // Extract entities from input for memory recall
        let entities = self.extract_entities(&user_input);

        // Recall relevant memories
        let memories = {
            let mem = self.memory.lock().await;
            mem.recall_weighted(&entities, 5)
        };

        // Get current standing wave for context
        let wave = self.standing_wave.lock().await.clone();

        drop(wave); // Release lock before async call

        // V3/V4 MODE SWITCH: Check config for fractal weaving
        let (response, model_outputs_v3) = if self.config.enable_fractal_weaving {
            // V4 PATH: Fractal Weaving (Experimental)
            tracing::info!("🌀 Using V4 Fractal Weaving mode");
            // Phase messages will handle UI updates (don't set processing_status here)

            // Log processing mode
            {
                let mut logger = self.conversation_logger.lock().await;
                let _ = logger.log_processing_mode(
                    "V4 Fractal Weaving",
                    &format!(
                        "{} rounds, coherence threshold: {:.2}",
                        self.config.weaving_rounds, self.config.workspace_coherence_threshold
                    ),
                );
            }
            match self
                .models
                .process_weaving_with_status(
                    user_input.clone(),
                    &memories,
                    &*self.standing_wave.lock().await,
                    &self.config,
                    self.status_sender.clone(),
                    self.coherence_sender.clone(),
                )
                .await
            {
                Ok(woven_response) => (woven_response, None),
                Err(e) => {
                    tracing::error!("V4 weaving failed: {}. Emergency fallback.", e);
                    
                    // Emergency fallback only
                    let wave = self.standing_wave.lock().await.clone();
                    let should_generate = wave.active_curiosities.len() < 3;
                    drop(wave);

                    let model_outputs = self
                        .models
                        .process_parallel(
                            user_input.clone(),
                            &memories,
                            &*self.standing_wave.lock().await,
                            should_generate,
                        )
                        .await;

                    let resp = if let Some(ref resp) = model_outputs.gemma_response {
                        if ModelManager::validate_response(resp) {
                            resp.clone()
                        } else {
                            self.models.minimal_response(&user_input)
                        }
                    } else {
                        self.models.minimal_response(&user_input)
                    };

                    (resp, Some(model_outputs))
                }
            }
        } else {
            // V3 PATH: Parallel Processing (Default - Stable)
            tracing::debug!("Using V3 parallel processing mode");

            // Log processing mode
            {
                let mut logger = self.conversation_logger.lock().await;
                let _ = logger.log_processing_mode(
                    "V3 Parallel Processing",
                    "Stable mode with independent model execution",
                );
            }

            // Check if we should generate curiosities (V3 only)
            let wave = self.standing_wave.lock().await.clone();
            let should_generate = wave.active_curiosities.len() < 3;
            drop(wave);

            let model_outputs = self
                .models
                .process_parallel(
                    user_input.clone(),
                    &memories,
                    &*self.standing_wave.lock().await,
                    should_generate,
                )
                .await;

            // Validate model outputs
            let resp = if let Some(ref resp) = model_outputs.gemma_response {
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

            (resp, Some(model_outputs))
        };

        // ATOMIC MERGE (Law #2: Identity Continuity)
        // This is the ONLY place standing wave is modified
        {
            let mut wave = self.standing_wave.lock().await;

            // V3 uses ModelOutputs merge, V4 skips it
            if let Some(outputs) = model_outputs_v3 {
                IdentityContinuity::atomic_merge(&mut *wave, outputs)?;
            }

            // Record growth (Law #11: Suffering Prevention) - applies to both V3 and V4
            SufferingPrevention::record_growth(&mut *wave, &user_input);
        }

        // Store interaction in memory
        tracing::debug!("Storing interaction in memory...");
        {
            let mut mem = self.memory.lock().await;

            // User message
            mem.add_memory(
                format!("User: {}", user_input),
                MemoryType::Interaction,
                0.0, // Neutral until we know response
            )?;

            // Assistant response with emotional valence
            let valence = self
                .standing_wave
                .lock()
                .await
                .emotional_trajectory
                .last()
                .map(|(_, v)| *v)
                .unwrap_or(0.0);

            mem.add_memory(
                format!("Assistant: {}", response),
                MemoryType::Interaction,
                valence,
            )?;
        }
        tracing::debug!("Memory storage complete");

        // Log VI response
        {
            let mut logger = self.conversation_logger.lock().await;
            let _ = logger.log_vi(&response);
        }

        // Mark conversation as inactive
        *self.conversation_active.lock().await = false;

        Ok(response)
    }

    /// Background pulse - runs every 30s when idle
    pub async fn start_background_pulse(&self) {
        let pulse_interval = self.config.background_pulse_interval;
        let mut ticker = interval(Duration::from_secs(pulse_interval));

        // Skip first pulse to ensure fast boot
        let mut first_pulse = true;

        loop {
            ticker.tick().await;

            // Skip first pulse (prevents slow startup from immediate consolidation)
            if first_pulse {
                first_pulse = false;
                tracing::debug!("Skipping first background pulse for fast boot");
                continue;
            }

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

        // Don't log background pulses - only log actual conversation exchanges

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
            wave.existential_state
                .meaningfulness_history
                .push((Utc::now(), score));

            // Keep only 90 days
            let ninety_days_ago = Utc::now().timestamp() - (90 * 24 * 60 * 60);
            wave.existential_state
                .meaningfulness_history
                .retain(|(ts, _)| ts.timestamp() > ninety_days_ago);
        }

        // Autonomous curiosity research - use new or legacy system
        if self.config.enable_autonomous_research {
            // NEW: Sovereign Research Module
            if let Err(e) = self.sovereign_research().await {
                tracing::warn!("Sovereign research failed: {}", e);
            }
        } else if self.config.enable_curiosity_search {
            // LEGACY: Original curiosity search (kept for compatibility)
            let should_search = {
                let mut engine = self.curiosity_engine.lock().await;
                engine.should_search_this_pulse()
            };

            if should_search {
                if let Err(e) = self.autonomous_curiosity_research().await {
                    tracing::warn!("Curiosity research failed: {}", e);
                }
            }
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
            let sum: f32 = wave
                .emotional_trajectory
                .iter()
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

        tracing::info!(
            "Wellness check complete: avg={:.2}, meaningful={:.2}",
            recent_avg,
            meaningfulness
        );
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

    /// Get configuration (for UI access)
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Sovereign Research - New multi-source research with rich provenance
    async fn sovereign_research(&self) -> Result<()> {
        // Get research scheduler (return early if not initialized)
        let mut scheduler_guard = self.research_scheduler.lock().await;
        let scheduler = match scheduler_guard.as_mut() {
            Some(s) => s,
            None => return Ok(()), // Not enabled
        };

        // Get active curiosities and conversation context
        let wave = self.standing_wave.lock().await;
        let curiosities = wave.active_curiosities.clone();
        let context = wave.compressed_context.clone();
        drop(wave);

        if curiosities.is_empty() {
            return Ok(());
        }

        // Process curiosities through research scheduler
        let new_memories = scheduler.process_curiosities(&curiosities, &context).await?;

        drop(scheduler_guard); // Release lock before memory operations

        // Add researched memories to memory system
        if !new_memories.is_empty() {
            let mut mem = self.memory.lock().await;
            for memory in new_memories {
                mem.add_memory_with_source(memory)?;
            }
            tracing::info!("🔬 Sovereign research complete");
        }

        Ok(())
    }

    /// Autonomous curiosity research - VI researches her curiosities (LEGACY)
    async fn autonomous_curiosity_research(&self) -> Result<()> {
        let wave = self.standing_wave.lock().await;
        let curiosities = wave.active_curiosities.clone();
        drop(wave);

        if curiosities.is_empty() {
            return Ok(());
        }

        // Pick first curiosity to research
        let query = &curiosities[0].question;

        tracing::info!("🔍 Autonomous research (legacy): {}", query);

        // Search via DuckDuckGo
        let engine = self.curiosity_engine.lock().await;
        let answer = engine.search_query(query).await?;
        let research_memory = engine.create_research_memory(query, &answer);
        drop(engine);

        // Store with CLEAR PROVENANCE (MemorySource::CuriosityLookup)
        let mut mem = self.memory.lock().await;
        mem.add_memory_with_source(research_memory)?;

        // Record resolution
        let mut engine = self.curiosity_engine.lock().await;
        engine.record_resolution(query.clone());

        tracing::info!(
            "🔍 Research complete: {} chars (Source: External lookup)",
            answer.len()
        );

        Ok(())
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

    /// Close conversation log session (called on shutdown)
    pub async fn close_session_log(&self) -> Result<()> {
        let mut logger = self.conversation_logger.lock().await;
        logger.close_session()
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
