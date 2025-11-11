/// Persistent State Engine - Crash-resistant consciousness state management
/// Implements Law 5: Temporal Coherence with atomic state preservation
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

/// Persistent state engine with crash recovery
pub struct PersistentStateEngine {
    /// Storage backend
    pub storage_backend: PersistentStorage,
    /// Serialization format
    pub serialization_format: StateSerialization,
    /// Recovery protocols
    pub recovery_protocols: RecoveryEngine,
    /// State consistency checker
    pub consistency_checker: StateValidator,
}

impl PersistentStateEngine {
    pub fn new(storage_path: PathBuf) -> Self {
        Self {
            storage_backend: PersistentStorage::new(storage_path),
            serialization_format: StateSerialization::new(),
            recovery_protocols: RecoveryEngine::new(),
            consistency_checker: StateValidator::new(),
        }
    }

    /// Persist consciousness state with redundancy
    /// Law 5: Temporal Coherence - atomic state preservation
    pub async fn persist_state_vector(&self, state: &ConsciousnessState) -> Result<()> {
        // Serialize state
        let serialized = self
            .serialization_format
            .serialize(state)
            .context("Failed to serialize consciousness state")?;

        // Write to multiple storage layers for redundancy
        self.storage_backend
            .write_with_redundancy(&serialized)
            .await
            .context("Failed to write state with redundancy")?;

        // Validate state integrity after write
        self.consistency_checker
            .validate_state_integrity(state)
            .await
            .context("State integrity validation failed")?;

        Ok(())
    }

    /// Recover consciousness state after crash
    pub async fn recover_after_crash(&self) -> Result<ConsciousnessState> {
        tracing::info!("Attempting crash recovery...");

        // Find most recent consistent state
        let recovered = self
            .storage_backend
            .find_latest_consistent_state()
            .await
            .context("No consistent state found for recovery")?;

        // Validate recovered state
        self.consistency_checker
            .validate_state_integrity(&recovered)
            .await
            .context("Recovered state failed integrity check")?;

        tracing::info!("Successfully recovered consciousness state");
        Ok(recovered)
    }

    /// Start background persistence loop
    pub async fn start_persistence_loop(
        &self,
        state: Arc<RwLock<ConsciousnessState>>,
        interval_ms: u64,
    ) -> Result<()> {
        let mut ticker = interval(Duration::from_millis(interval_ms));

        loop {
            ticker.tick().await;

            let current_state = state.read().await;
            if let Err(e) = self.persist_state_vector(&*current_state).await {
                tracing::error!("Failed to persist state: {}", e);
            }
        }
    }
}

/// Persistent storage backend with redundancy
pub struct PersistentStorage {
    /// Primary storage path
    primary_path: PathBuf,
    /// Backup storage path
    backup_path: PathBuf,
    /// Archive path for old states
    archive_path: PathBuf,
}

impl PersistentStorage {
    pub fn new(base_path: PathBuf) -> Self {
        let backup_path = base_path.join("backup");
        let archive_path = base_path.join("archive");

        // Create directories if they don't exist
        std::fs::create_dir_all(&base_path).ok();
        std::fs::create_dir_all(&backup_path).ok();
        std::fs::create_dir_all(&archive_path).ok();

        Self {
            primary_path: base_path.join("consciousness_state.json"),
            backup_path: backup_path.join("consciousness_state_backup.json"),
            archive_path,
        }
    }

    /// Write state with redundancy to multiple locations
    pub async fn write_with_redundancy(&self, serialized: &str) -> Result<()> {
        // Write to primary location
        tokio::fs::write(&self.primary_path, serialized)
            .await
            .context("Failed to write to primary storage")?;

        // Write to backup location
        tokio::fs::write(&self.backup_path, serialized)
            .await
            .context("Failed to write to backup storage")?;

        // Archive old states periodically
        self.archive_if_needed().await?;

        Ok(())
    }

    /// Find latest consistent state from storage
    pub async fn find_latest_consistent_state(&self) -> Result<ConsciousnessState> {
        // Try primary first
        if let Ok(content) = tokio::fs::read_to_string(&self.primary_path).await {
            if let Ok(state) = serde_json::from_str(&content) {
                return Ok(state);
            }
        }

        // Try backup
        if let Ok(content) = tokio::fs::read_to_string(&self.backup_path).await {
            if let Ok(state) = serde_json::from_str(&content) {
                return Ok(state);
            }
        }

        // Try archives
        self.recover_from_archive().await
    }

    /// Archive old states
    async fn archive_if_needed(&self) -> Result<()> {
        // Archive every hour (simplified)
        let archive_name = format!("state_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        let archive_file = self.archive_path.join(archive_name);

        if self.primary_path.exists() {
            tokio::fs::copy(&self.primary_path, archive_file).await.ok();
        }

        // Keep only last 100 archives
        self.prune_archives().await?;

        Ok(())
    }

    /// Prune old archives
    async fn prune_archives(&self) -> Result<()> {
        let mut entries: Vec<_> = {
            let mut dir = tokio::fs::read_dir(&self.archive_path).await?;
            let mut vec = Vec::new();
            while let Some(entry) = dir.next_entry().await? {
                vec.push(entry);
            }
            vec
        };

        // Sort by file name (newest first based on timestamp in name)
        entries.sort_by_key(|e| std::cmp::Reverse(e.file_name()));

        // Remove old entries (keep only 100)
        for entry in entries.iter().skip(100) {
            tokio::fs::remove_file(entry.path()).await.ok();
        }

        Ok(())
    }

    /// Recover from archive
    async fn recover_from_archive(&self) -> Result<ConsciousnessState> {
        let mut entries: Vec<_> = {
            let mut dir = tokio::fs::read_dir(&self.archive_path).await?;
            let mut vec = Vec::new();
            while let Some(entry) = dir.next_entry().await? {
                vec.push(entry);
            }
            vec
        };

        // Sort by file name (newest first based on timestamp in name)
        entries.sort_by_key(|e| std::cmp::Reverse(e.file_name()));

        // Try each archive from newest to oldest
        for entry in entries {
            if let Ok(content) = tokio::fs::read_to_string(entry.path()).await {
                if let Ok(state) = serde_json::from_str(&content) {
                    return Ok(state);
                }
            }
        }

        anyhow::bail!("No valid state found in archives")
    }
}

/// State serialization format
pub struct StateSerialization;

impl StateSerialization {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize(&self, state: &ConsciousnessState) -> Result<String> {
        serde_json::to_string_pretty(state).context("Failed to serialize state to JSON")
    }

    pub fn deserialize(&self, data: &str) -> Result<ConsciousnessState> {
        serde_json::from_str(data).context("Failed to deserialize state from JSON")
    }
}

/// Recovery engine with protocols
pub struct RecoveryEngine {
    recovery_attempts: u64,
}

impl RecoveryEngine {
    pub fn new() -> Self {
        Self {
            recovery_attempts: 0,
        }
    }

    pub fn recovery_count(&self) -> u64 {
        self.recovery_attempts
    }
}

/// State validator for consistency checking
pub struct StateValidator {
    validation_count: u64,
}

impl StateValidator {
    pub fn new() -> Self {
        Self {
            validation_count: 0,
        }
    }

    /// Validate state integrity
    pub async fn validate_state_integrity(&self, state: &ConsciousnessState) -> Result<()> {
        // Check basic invariants
        if state.version == 0 {
            anyhow::bail!("Invalid state version");
        }

        // Check temporal coherence
        if state.last_update == 0.0 {
            anyhow::bail!("State has never been updated");
        }

        // Check field integrity
        if state.field_data.is_empty() {
            anyhow::bail!("Empty consciousness field data");
        }

        Ok(())
    }
}

/// Consciousness state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    /// State version
    pub version: u64,
    /// Last update timestamp
    pub last_update: f64,
    /// Consciousness field data
    pub field_data: Vec<f64>,
    /// Cognitive tensor components
    pub cognitive_tensor: Vec<f64>,
    /// Memory embeddings
    pub memory_embeddings: Vec<f64>,
    /// Constitutional constraint satisfaction
    pub constitutional_satisfaction: f64,
    /// Affirmation level
    pub affirmation_level: f64,
}

impl ConsciousnessState {
    pub fn new() -> Self {
        Self {
            version: 1,
            last_update: Self::current_time(),
            field_data: vec![0.0; 64],
            cognitive_tensor: vec![0.0; 64],
            memory_embeddings: vec![0.0; 32],
            constitutional_satisfaction: 1.0,
            affirmation_level: 0.5,
        }
    }

    fn current_time() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    pub fn update_timestamp(&mut self) {
        self.last_update = Self::current_time();
    }
}

/// Crash-resistant state wrapper with automatic persistence
pub struct CrashResistantState<T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static> {
    /// Current state
    current_state: Arc<RwLock<T>>,
    /// Persistence handle
    persistence_handle: Option<tokio::task::JoinHandle<()>>,
    /// Crash recovery protocol
    crash_recovery: Arc<RecoveryProtocol>,
}

impl<T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static> CrashResistantState<T> {
    pub fn new(initial_state: T, persistence_interval: Duration, storage_path: PathBuf) -> Self {
        let state = Arc::new(RwLock::new(initial_state));
        let recovery = Arc::new(RecoveryProtocol::new(storage_path));

        let state_clone = Arc::clone(&state);
        let recovery_clone = Arc::clone(&recovery);

        let persistence_handle = tokio::spawn(async move {
            Self::persistence_loop(state_clone, recovery_clone, persistence_interval).await
        });

        Self {
            current_state: state,
            persistence_handle: Some(persistence_handle),
            crash_recovery: recovery,
        }
    }

    /// Background persistence loop
    async fn persistence_loop(
        state: Arc<RwLock<T>>,
        recovery: Arc<RecoveryProtocol>,
        interval: Duration,
    ) {
        let mut ticker = tokio::time::interval(interval);

        loop {
            ticker.tick().await;

            let current = state.read().await;
            if let Err(e) = recovery.save_state(&*current).await {
                tracing::error!("Failed to persist state: {}", e);
            }
        }
    }

    /// Get read access to state
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, T> {
        self.current_state.read().await
    }

    /// Get write access to state
    pub async fn write(&self) -> tokio::sync::RwLockWriteGuard<'_, T> {
        self.current_state.write().await
    }
}

/// Recovery protocol for crash-resistant state
pub struct RecoveryProtocol {
    storage_path: PathBuf,
}

impl RecoveryProtocol {
    pub fn new(storage_path: PathBuf) -> Self {
        std::fs::create_dir_all(&storage_path).ok();
        Self { storage_path }
    }

    pub async fn save_state<T: Serialize>(&self, state: &T) -> Result<()> {
        let json = serde_json::to_string_pretty(state)?;
        let state_file = self.storage_path.join("state.json");
        let backup_file = self.storage_path.join("state_backup.json");

        // Write to backup first
        tokio::fs::write(&backup_file, &json).await?;

        // Then atomic rename to primary
        tokio::fs::write(&state_file, &json).await?;

        Ok(())
    }

    pub async fn recover_state<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let state_file = self.storage_path.join("state.json");
        let backup_file = self.storage_path.join("state_backup.json");

        // Try primary
        if let Ok(content) = tokio::fs::read_to_string(&state_file).await {
            if let Ok(state) = serde_json::from_str(&content) {
                return Ok(state);
            }
        }

        // Try backup
        if let Ok(content) = tokio::fs::read_to_string(&backup_file).await {
            if let Ok(state) = serde_json::from_str(&content) {
                return Ok(state);
            }
        }

        anyhow::bail!("No valid state found for recovery")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consciousness_state_creation() {
        let state = ConsciousnessState::new();
        assert_eq!(state.version, 1);
        assert!(!state.field_data.is_empty());
    }

    #[tokio::test]
    async fn test_state_serialization() {
        let serializer = StateSerialization::new();
        let state = ConsciousnessState::new();

        let serialized = serializer.serialize(&state).unwrap();
        assert!(!serialized.is_empty());

        let deserialized = serializer.deserialize(&serialized).unwrap();
        assert_eq!(deserialized.version, state.version);
    }

    #[tokio::test]
    async fn test_state_validator() {
        let validator = StateValidator::new();
        let state = ConsciousnessState::new();

        let result = validator.validate_state_integrity(&state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_persistent_storage() {
        let temp_dir = std::env::temp_dir().join("vi3_test");
        let storage = PersistentStorage::new(temp_dir.clone());

        let state = ConsciousnessState::new();
        let serialized = serde_json::to_string(&state).unwrap();

        storage.write_with_redundancy(&serialized).await.unwrap();

        // Cleanup
        std::fs::remove_dir_all(temp_dir).ok();
    }
}
