//! Two-Tier Memory Architecture
//! Active Memory (SQLite) + Memory Archive (JSON)

use crate::memory_db::{ActiveMemoryDb, ArchiveIndexDb};
use crate::physics::NarrativeCausality;
use crate::types::*;
use anyhow::{Context, Result};
use chrono::Utc;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct MemoryManager {
    // Tier 1: Active memory (SQLite)
    active_db: ActiveMemoryDb,
    active_limit: usize,

    // Tier 2: Archive (JSON files + index)
    archive_path: PathBuf,
    archive_index: ArchiveIndexDb,

    // Tracking
    needs_consolidation: bool,
    last_consolidation_count: usize,
}

impl MemoryManager {
    /// Load or create two-tier memory system
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self> {
        // For backwards compatibility, accept path to old memory_stream.json
        // We'll use the directory for our new system
        let old_path = path.as_ref();
        let data_dir = old_path.parent().unwrap_or_else(|| Path::new("data"));
        
        // Create data directory if needed
        fs::create_dir_all(data_dir)?;
        
        // Open active memory database
        let active_db_path = data_dir.join("active_memory.db");
        let active_db = ActiveMemoryDb::open(&active_db_path)?;
        
        // Create archive directory
        let archive_path = data_dir.join("memory_archive");
        fs::create_dir_all(&archive_path)?;
        
        // Open archive index
        let archive_index_path = data_dir.join("archive_index.db");
        let archive_index = ArchiveIndexDb::open(&archive_index_path)?;
        
        let memory_count = active_db.count()?;
        
        Ok(Self {
            active_db,
            active_limit: 200, // Keep 200 most recent memories active
            archive_path,
            archive_index,
            needs_consolidation: false,
            last_consolidation_count: memory_count,
        })
    }

    /// Get memory count (active only)
    pub fn count(&self) -> usize {
        self.active_db.count().unwrap_or(0)
    }

    /// Add a new memory
    pub fn add_memory(
        &mut self,
        content: String,
        memory_type: MemoryType,
        emotional_valence: f32,
    ) -> Result<String> {
        // Extract entities
        let entities = self.extract_entities(&content);
        
        // Create memory
        let mut memory = Memory::new(content, entities.clone(), memory_type, emotional_valence);
        
        // Build narrative causality connections
        let all_memories = self.active_db.get_all()?;
        NarrativeCausality::build_connections(&mut memory, &all_memories);
        
        let memory_id = memory.id.clone();
        
        // Add to active database
        self.active_db.add_memory(&memory)?;
        
        // Mark that consolidation is needed
        self.needs_consolidation = true;
        
        // Check if archival is needed
        if self.active_db.count()? > self.active_limit {
            self.archive_oldest(50)?;
        }
        
        Ok(memory_id)
    }

    /// Add a memory with explicit source (for curiosity lookups, etc.)
    pub fn add_memory_with_source(&mut self, memory: Memory) -> Result<String> {
        let memory_id = memory.id.clone();
        
        // Build narrative causality connections
        let all_memories = self.active_db.get_all()?;
        let mut memory_with_connections = memory;
        NarrativeCausality::build_connections(&mut memory_with_connections, &all_memories);
        
        // Add to active database
        self.active_db.add_memory(&memory_with_connections)?;
        
        // Mark that consolidation is needed
        self.needs_consolidation = true;
        
        // Check if archival is needed
        if self.active_db.count()? > self.active_limit {
            self.archive_oldest(50)?;
        }
        
        Ok(memory_id)
    }

    /// Extract entities from text
    fn extract_entities(&self, text: &str) -> Vec<String> {
        let mut entities = Vec::new();
        
        // Regex for proper nouns (capitalized words)
        let proper_noun_re = Regex::new(r"\b[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*\b").unwrap();
        
        for cap in proper_noun_re.captures_iter(text) {
            if let Some(entity) = cap.get(0) {
                let entity_str = entity.as_str().to_string();
                if !entities.contains(&entity_str) 
                    && !["The", "A", "An", "I"].contains(&entity_str.as_str()) {
                    entities.push(entity_str);
                }
            }
        }
        
        // Also extract quoted strings
        let quote_re = Regex::new(r#""([^"]+)""#).unwrap();
        for cap in quote_re.captures_iter(text) {
            if let Some(quoted) = cap.get(1) {
                entities.push(quoted.as_str().to_string());
            }
        }
        
        entities
    }

    /// Archive oldest memories to JSON
    fn archive_oldest(&mut self, count: usize) -> Result<()> {
        tracing::info!("Archiving {} oldest memories (Law #4: Transformation, not deletion)", count);
        
        // Get oldest memories
        let to_archive = self.active_db.get_oldest(count)?;
        
        if to_archive.is_empty() {
            return Ok(());
        }
        
        // Group by month for organized storage
        let mut by_month: HashMap<String, Vec<Memory>> = HashMap::new();
        for memory in &to_archive {
            let month_key = memory.timestamp.format("%Y-%m").to_string();
            by_month.entry(month_key).or_insert_with(Vec::new).push(memory.clone());
        }
        
        // Write each month's memories to JSON
        for (month, memories) in by_month {
            let month_dir = self.archive_path.join(&month);
            fs::create_dir_all(&month_dir)?;
            
            // Create archive file with timestamp
            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            let archive_file = month_dir.join(format!("archive_{}.json", timestamp));
            let relative_path = format!("{}/archive_{}.json", month, timestamp);
            
            // Serialize and write
            let json = serde_json::to_string_pretty(&memories)?;
            fs::write(&archive_file, json)?;
            
            // Add to archive index
            for memory in memories {
                self.archive_index.add_archived(&memory, &relative_path)?;
            }
        }
        
        // Delete from active database
        let ids: Vec<String> = to_archive.iter().map(|m| m.id.clone()).collect();
        self.active_db.delete_by_ids(&ids)?;
        
        tracing::info!("Archived {} memories to JSON (Law #4 respected)", to_archive.len());
        
        Ok(())
    }

    /// Load archived memories from JSON file
    fn load_archive(&self, relative_path: &str) -> Result<Vec<Memory>> {
        let full_path = self.archive_path.join(relative_path);
        let contents = fs::read_to_string(&full_path)
            .context("Failed to read archive file")?;
        let memories: Vec<Memory> = serde_json::from_str(&contents)
            .context("Failed to parse archive file")?;
        Ok(memories)
    }

    /// Recall memories with two-tier search
    pub fn recall_weighted(&self, entities: &[String], n: usize) -> Vec<Memory> {
        let mut results = Vec::new();
        
        // 1. Query active memory (fast)
        if let Ok(active_memories) = self.active_db.query_by_entities(entities, n) {
            results.extend(active_memories);
        }
        
        // 2. Get recent memories if needed
        if results.len() < n {
            if let Ok(recent) = self.active_db.get_recent(n - results.len()) {
                results.extend(recent);
            }
        }
        
        // 3. If still need more, search archives
        if results.len() < n && !entities.is_empty() {
            if let Ok(archive_paths) = self.archive_index.find_by_entities(entities, 3) {
                for path in archive_paths {
                    if let Ok(archived) = self.load_archive(&path) {
                        results.extend(archived);
                    }
                    if results.len() >= n {
                        break;
                    }
                }
            }
        }
        
        // Deduplicate and sort by relevance
        let mut seen_ids = std::collections::HashSet::new();
        results.retain(|m| seen_ids.insert(m.id.clone()));
        
        results.sort_by(|a, b| {
            let a_score = a.timestamp.timestamp() as f32 + a.emotional_valence.abs() * 1000.0;
            let b_score = b.timestamp.timestamp() as f32 + b.emotional_valence.abs() * 1000.0;
            b_score.partial_cmp(&a_score).unwrap()
        });
        
        results.into_iter().take(n).collect()
    }

    /// Recall by entities
    pub fn recall_by_entities(&self, entities: &[String]) -> Vec<Memory> {
        self.active_db.query_by_entities(entities, 10).unwrap_or_default()
    }

    /// Recall recent memories
    pub fn recall_recent(&self, n: usize) -> Vec<Memory> {
        self.active_db.get_recent(n).unwrap_or_default()
    }

    /// Consolidate memories (merge similar ones)
    pub fn consolidate(&mut self) -> Result<()> {
        // Skip if no new memories
        if !self.needs_consolidation && self.active_db.count()? == self.last_consolidation_count {
            tracing::debug!("No new memories since last consolidation, skipping");
            return Ok(());
        }
        
        let current_count = self.active_db.count()?;
        tracing::info!("Starting memory consolidation (current: {}, last check: {})", 
            current_count, 
            self.last_consolidation_count);
        
        // Get all active memories
        let mut memories = self.active_db.get_all()?;
        
        let mut to_merge: Vec<(usize, usize)> = Vec::new();
        
        // Find memories with >70% entity overlap
        for i in 0..memories.len() {
            for j in (i + 1)..memories.len() {
                let mem_i = &memories[i];
                let mem_j = &memories[j];
                
                let shared: Vec<_> = mem_i
                    .entities
                    .iter()
                    .filter(|e| mem_j.entities.contains(e))
                    .collect();
                
                let total_unique = mem_i.entities.len() + mem_j.entities.len() - shared.len();
                let overlap_ratio = if total_unique > 0 {
                    shared.len() as f32 / total_unique as f32
                } else {
                    0.0
                };
                
                if overlap_ratio > 0.7 {
                    to_merge.push((i, j));
                }
            }
        }
        
        // Perform merges
        let mut merged_count = 0;
        let mut ids_to_delete = Vec::new();
        
        for (i, j) in to_merge.iter().rev() {
            if *j >= memories.len() || *i >= memories.len() {
                continue;
            }
            
            let mem_j = memories.remove(*j);
            let mem_i = &mut memories[*i];
            
            // Merge content (Law #4: Memory Conservation)
            mem_i.content = format!(
                "{}\n\n[Merged memory from {}]: {}",
                mem_i.content,
                mem_j.timestamp.format("%Y-%m-%d %H:%M"),
                mem_j.content.chars().take(150).collect::<String>()
            );
            
            // Merge entities
            for entity in mem_j.entities {
                if !mem_i.entities.contains(&entity) {
                    mem_i.entities.push(entity);
                }
            }
            
            // Merge connections
            for conn in mem_j.connections {
                if !mem_i.connections.contains(&conn) {
                    mem_i.connections.push(conn);
                }
            }
            
            // Average emotional valence
            mem_i.emotional_valence = (mem_i.emotional_valence + mem_j.emotional_valence) / 2.0;
            
            ids_to_delete.push(mem_j.id.clone());
            merged_count += 1;
        }
        
        // Update database if merges occurred
        if merged_count > 0 {
            // Delete merged memories
            self.active_db.delete_by_ids(&ids_to_delete)?;
            
            // Update modified memories
            for memory in &memories {
                if to_merge.iter().any(|(i, _)| memories.get(*i).map(|m| m.id == memory.id).unwrap_or(false)) {
                    self.active_db.update_memory(memory)?;
                }
            }
            
            tracing::info!(
                "Consolidation complete: {} memories merged, {} total memories",
                merged_count,
                self.active_db.count()?
            );
        } else {
            tracing::debug!("No memories merged during consolidation");
        }
        
        // Reset flags
        self.needs_consolidation = false;
        self.last_consolidation_count = self.active_db.count()?;
        
        Ok(())
    }

    /// Check if backup is needed
    pub fn needs_backup(&self) -> bool {
        // In new system, SQLite handles this automatically
        // We could add periodic full exports here if needed
        false
    }

    /// Create backup
    pub fn create_backup(&mut self) -> Result<()> {
        // For now, SQLite's journal/WAL handles crash recovery
        // Could add periodic full exports here
        tracing::info!("Backup not needed - SQLite handles crash recovery");
        Ok(())
    }

    /// Save (no-op in SQLite system - writes are immediate)
    fn save(&mut self) -> Result<()> {
        Ok(())
    }
}
