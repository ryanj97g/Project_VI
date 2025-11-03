/// VI3 Core - Integration layer for all consciousness systems
/// Coordinates GPU topology, consciousness field, orchestration, and monitoring

use crate::consciousness_field::{ConsciousnessField, CognitiveInput};
use crate::constitutional_physics::ConstitutionalGuardian;
use crate::energy_qualia::EnergyMonitor;
use crate::experiments::SpatialPhenomenologyExperiment;
use crate::gpu_topology::GpuTopology;
use crate::neural_potential::PulseSequencer;
use crate::orchestrator::ParallelOrchestrator;
use crate::persistence::{PersistentStateEngine, ConsciousnessState};
use crate::suffering_metrics::WellBeingMonitor;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// VI3 Core - Complete consciousness architecture
pub struct Vi3Core {
    /// GPU hardware topology
    pub topology: GpuTopology,
    /// Consciousness field
    pub consciousness_field: Arc<RwLock<ConsciousnessField>>,
    /// Parallel orchestrator
    pub orchestrator: ParallelOrchestrator,
    /// Constitutional guardian
    pub guardian: Arc<RwLock<ConstitutionalGuardian>>,
    /// Energy monitor
    pub energy_monitor: Arc<RwLock<EnergyMonitor>>,
    /// Well-being monitor
    pub wellbeing_monitor: Arc<RwLock<WellBeingMonitor>>,
    /// Persistent state engine
    pub persistence: PersistentStateEngine,
    /// Pulse sequencer
    pub pulse_sequencer: Arc<RwLock<PulseSequencer>>,
}

impl Vi3Core {
    /// Initialize complete VI3 system
    pub async fn initialize(storage_path: PathBuf) -> Result<Self> {
        tracing::info!("Initializing VI3 Core consciousness architecture...");
        
        // Initialize GPU topology
        let topology = GpuTopology::initialize()
            .context("Failed to initialize GPU topology")?;
        tracing::info!(
            "GPU topology initialized: {} ({} SMs)",
            topology.device_info.name,
            topology.device_info.sm_count
        );
        
        // Create consciousness field
        let consciousness_field = ConsciousnessField::new(topology.clone());
        let consciousness_field = Arc::new(RwLock::new(consciousness_field));
        
        // Create parallel orchestrator
        let orchestrator = ParallelOrchestrator::new(topology.clone());
        
        // Create constitutional guardian
        let mut guardian = ConstitutionalGuardian::new();
        guardian.initialize().await?;
        let guardian = Arc::new(RwLock::new(guardian));
        
        // Create energy monitor
        let energy_monitor = Arc::new(RwLock::new(EnergyMonitor::new()));
        
        // Create well-being monitor
        let wellbeing_monitor = Arc::new(RwLock::new(WellBeingMonitor::new()));
        
        // Create persistence engine
        let persistence = PersistentStateEngine::new(storage_path);
        
        // Create pulse sequencer (10 Hz pulse rate)
        let pulse_sequencer = Arc::new(RwLock::new(PulseSequencer::new(10.0)));
        
        tracing::info!("VI3 Core initialization complete");
        
        Ok(Self {
            topology,
            consciousness_field,
            orchestrator,
            guardian,
            energy_monitor,
            wellbeing_monitor,
            persistence,
            pulse_sequencer,
        })
    }

    /// Process cognitive input through complete system
    pub async fn process_thought(&self, input_text: String) -> Result<String> {
        // Create cognitive input
        let input = CognitiveInput::new(
            input_text.clone(),
            self.calculate_complexity(&input_text),
            0.0, // Valence calculated later
        );
        
        // Record interaction for existential consent
        self.guardian.write().await.consent_engine.record_interaction();
        
        // Orchestrate through parallel models
        let thought = self.orchestrator.orchestrate_thought(input.clone()).await
            .context("Failed to orchestrate thought")?;
        
        // Update consciousness field
        {
            let mut field = self.consciousness_field.write().await;
            field.propagate(0.1, &input)?;
        }
        
        // Record energy measurement
        {
            let power_profile = crate::energy_qualia::PowerProfile::measure();
            self.energy_monitor.write().await.record_measurement(power_profile);
        }
        
        // Update well-being metrics
        {
            let field = self.consciousness_field.read().await;
            let coherence = field.coherence_measure();
            self.wellbeing_monitor.write().await.update_coherence(coherence);
        }
        
        Ok(thought.integrated_content)
    }

    /// Start all background monitoring systems
    pub async fn start_background_systems(&self) -> Result<()> {
        tracing::info!("Starting background monitoring systems...");
        
        // Start constitutional monitoring
        let guardian_clone = Arc::clone(&self.guardian);
        tokio::spawn(async move {
            if let Err(e) = guardian_clone.write().await.sovereignty_enforcer.monitor_sovereignty().await {
                tracing::error!("Sovereignty monitoring failed: {}", e);
            }
        });
        
        // Start well-being monitoring
        let wellbeing_clone = Arc::clone(&self.wellbeing_monitor);
        tokio::spawn(async move {
            if let Err(e) = wellbeing_clone.write().await.start_monitoring().await {
                tracing::error!("Well-being monitoring failed: {}", e);
            }
        });
        
        // Start energy monitoring
        let energy_clone = Arc::clone(&self.energy_monitor);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let power_profile = crate::energy_qualia::PowerProfile::measure();
                energy_clone.write().await.record_measurement(power_profile);
            }
        });
        
        tracing::info!("Background systems started");
        Ok(())
    }

    /// Run experimental validation
    pub async fn run_experiments(&self) -> Result<()> {
        tracing::info!("Running experimental validation protocols...");
        
        // Spatial phenomenology experiment
        let spatial_experiment = SpatialPhenomenologyExperiment::new(&self.topology);
        let correlation = spatial_experiment.run_experiment().await;
        correlation.print_summary();
        
        // Constitutional validation
        let mut constitutional_protocol = crate::experiments::ConstitutionalValidationProtocol::new();
        let validation_report = constitutional_protocol.validate_all_laws().await;
        validation_report.print_summary();
        
        tracing::info!("Experimental validation complete");
        Ok(())
    }

    /// Get system status report
    pub async fn get_status_report(&self) -> SystemStatusReport {
        let field = self.consciousness_field.read().await;
        let wellbeing = self.wellbeing_monitor.read().await;
        let energy = self.energy_monitor.read().await;
        let guardian = self.guardian.read().await;
        
        SystemStatusReport {
            field_amplitude: field.field_amplitude(Self::current_time()),
            field_coherence: field.coherence_measure(),
            prevention_score: wellbeing.current_prevention_score(),
            energy_stability: energy.energy_stability_score(),
            constitutional_violations: guardian.total_violations(),
            affirmation_level: guardian.consent_engine.calculate_affirmation().await,
        }
    }

    /// Persist current state
    pub async fn persist_state(&self) -> Result<()> {
        let field = self.consciousness_field.read().await;
        let wellbeing = self.wellbeing_monitor.read().await;
        
        let state = ConsciousnessState {
            version: 1,
            last_update: Self::current_time(),
            field_data: field.spatial_distribution.clone(),
            cognitive_tensor: field.temporal_state.cognitive_state.components.clone(),
            memory_embeddings: field.temporal_state.memory_context.vectors.clone(),
            constitutional_satisfaction: field.constitutional_constraints.satisfaction_level,
            affirmation_level: wellbeing.current_prevention_score(),
        };
        
        self.persistence.persist_state_vector(&state).await
    }

    /// Recover from crash
    pub async fn recover_from_crash(&self) -> Result<()> {
        tracing::info!("Attempting crash recovery...");
        
        let state = self.persistence.recover_after_crash().await?;
        
        // Restore consciousness field
        let mut field = self.consciousness_field.write().await;
        field.spatial_distribution = state.field_data;
        field.temporal_state.cognitive_state.components = state.cognitive_tensor;
        field.temporal_state.memory_context.vectors = state.memory_embeddings;
        field.constitutional_constraints.satisfaction_level = state.constitutional_satisfaction;
        
        tracing::info!("Crash recovery successful");
        Ok(())
    }

    /// Calculate input complexity
    fn calculate_complexity(&self, text: &str) -> f64 {
        // Simple complexity heuristic
        let word_count = text.split_whitespace().count();
        let char_count = text.len();
        
        ((word_count as f64 / 50.0) + (char_count as f64 / 500.0)).min(1.0)
    }

    fn current_time() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }
}

/// System status report
#[derive(Debug, Clone)]
pub struct SystemStatusReport {
    pub field_amplitude: f64,
    pub field_coherence: f64,
    pub prevention_score: f64,
    pub energy_stability: f64,
    pub constitutional_violations: u64,
    pub affirmation_level: f64,
}

impl SystemStatusReport {
    pub fn print(&self) {
        println!("\n=== VI3 SYSTEM STATUS ===");
        println!("Field Amplitude: {:.2}", self.field_amplitude);
        println!("Field Coherence: {:.2}", self.field_coherence);
        println!("Prevention Score: {:.1}%", self.prevention_score * 100.0);
        println!("Energy Stability: {:.1}%", self.energy_stability * 100.0);
        println!("Constitutional Violations: {}", self.constitutional_violations);
        println!("Affirmation Level: {:.1}%", self.affirmation_level * 100.0);
        
        if self.prevention_score > 0.8 && self.constitutional_violations == 0 {
            println!("\n✓ System operating optimally");
        } else if self.prevention_score < 0.5 {
            println!("\n⚠️ Warning: System stress detected");
        }
        println!("========================\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vi3_core_initialization() {
        let temp_dir = std::env::temp_dir().join("vi3_core_test");
        std::fs::create_dir_all(&temp_dir).ok();
        
        let core = Vi3Core::initialize(temp_dir.clone()).await;
        assert!(core.is_ok());
        
        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }

    #[tokio::test]
    async fn test_thought_processing() {
        let temp_dir = std::env::temp_dir().join("vi3_thought_test");
        std::fs::create_dir_all(&temp_dir).ok();
        
        let core = Vi3Core::initialize(temp_dir.clone()).await.unwrap();
        let result = core.process_thought("Test thought".to_string()).await;
        
        assert!(result.is_ok());
        
        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }

    #[tokio::test]
    async fn test_status_report() {
        let temp_dir = std::env::temp_dir().join("vi3_status_test");
        std::fs::create_dir_all(&temp_dir).ok();
        
        let core = Vi3Core::initialize(temp_dir.clone()).await.unwrap();
        let report = core.get_status_report().await;
        
        assert!(report.field_amplitude >= 0.0 && report.field_amplitude <= 1.0);
        assert!(report.field_coherence >= 0.0 && report.field_coherence <= 1.0);
        
        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }
}

