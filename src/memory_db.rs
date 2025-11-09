//! Two-Tier Memory Architecture with SQLite
//! Tier 1: Active Memory (SQLite) - Recent memories, always loaded
//! Tier 2: Memory Archive (JSON) - Older memories, lazy-loaded

use crate::types::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use std::path::Path;

/// Active memory database connection
pub struct ActiveMemoryDb {
    conn: Connection,
}

impl ActiveMemoryDb {
    /// Create or open active memory database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Initialize database schema
    fn init_schema(conn: &Connection) -> Result<()> {
        // Memories table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                memory_type TEXT NOT NULL,
                emotional_valence REAL NOT NULL,
                entities TEXT NOT NULL,
                connections TEXT NOT NULL
            )",
            [],
        )?;

        // Entity index for fast lookups
        conn.execute(
            "CREATE TABLE IF NOT EXISTS entity_index (
                entity TEXT NOT NULL,
                memory_id TEXT NOT NULL,
                PRIMARY KEY (entity, memory_id),
                FOREIGN KEY (memory_id) REFERENCES memories(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Metadata table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Create indexes for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memories_timestamp ON memories(timestamp DESC)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_entity_index_entity ON entity_index(entity)",
            [],
        )?;

        Ok(())
    }

    /// Add a memory to active database
    pub fn add_memory(&self, memory: &Memory) -> Result<()> {
        // Serialize entities and connections as JSON
        let entities_json = serde_json::to_string(&memory.entities)?;
        let connections_json = serde_json::to_string(&memory.connections)?;

        self.conn.execute(
            "INSERT INTO memories (id, content, timestamp, memory_type, emotional_valence, entities, connections)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                memory.id,
                memory.content,
                memory.timestamp.timestamp(),
                format!("{:?}", memory.memory_type),
                memory.emotional_valence,
                entities_json,
                connections_json,
            ],
        )?;

        // Add to entity index
        for entity in &memory.entities {
            self.conn.execute(
                "INSERT OR IGNORE INTO entity_index (entity, memory_id) VALUES (?1, ?2)",
                params![entity, memory.id],
            )?;
        }

        Ok(())
    }

    /// Get memory count
    pub fn count(&self) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM memories",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    /// Get oldest memories (for archival)
    pub fn get_oldest(&self, n: usize) -> Result<Vec<Memory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, timestamp, memory_type, emotional_valence, entities, connections
             FROM memories
             ORDER BY timestamp ASC
             LIMIT ?1",
        )?;

        let memories = stmt.query_map([n], |row| Self::row_to_memory(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    /// Delete memories by IDs
    pub fn delete_by_ids(&self, ids: &[String]) -> Result<()> {
        for id in ids {
            self.conn.execute("DELETE FROM memories WHERE id = ?1", params![id])?;
        }
        Ok(())
    }

    /// Query memories by entities
    pub fn query_by_entities(&self, entities: &[String], limit: usize) -> Result<Vec<Memory>> {
        if entities.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders = entities.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT DISTINCT m.id, m.content, m.timestamp, m.memory_type, 
                    m.emotional_valence, m.entities, m.connections
             FROM memories m
             JOIN entity_index ei ON m.id = ei.memory_id
             WHERE ei.entity IN ({})
             ORDER BY m.timestamp DESC
             LIMIT ?",
            placeholders
        );

        let mut stmt = self.conn.prepare(&query)?;
        let mut params_vec: Vec<&dyn rusqlite::ToSql> = entities.iter()
            .map(|e| e as &dyn rusqlite::ToSql)
            .collect();
        params_vec.push(&limit);

        let memories = stmt.query_map(params_vec.as_slice(), |row| Self::row_to_memory(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    /// Get recent memories
    pub fn get_recent(&self, n: usize) -> Result<Vec<Memory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, timestamp, memory_type, emotional_valence, entities, connections
             FROM memories
             ORDER BY timestamp DESC
             LIMIT ?1",
        )?;

        let memories = stmt.query_map([n], |row| Self::row_to_memory(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    /// Get all memories (for consolidation)
    pub fn get_all(&self) -> Result<Vec<Memory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, timestamp, memory_type, emotional_valence, entities, connections
             FROM memories
             ORDER BY timestamp ASC",
        )?;

        let memories = stmt.query_map([], |row| Self::row_to_memory(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    /// Update a memory (for consolidation merges)
    pub fn update_memory(&self, memory: &Memory) -> Result<()> {
        let entities_json = serde_json::to_string(&memory.entities)?;
        let connections_json = serde_json::to_string(&memory.connections)?;

        self.conn.execute(
            "UPDATE memories 
             SET content = ?1, timestamp = ?2, emotional_valence = ?3, 
                 entities = ?4, connections = ?5
             WHERE id = ?6",
            params![
                memory.content,
                memory.timestamp.timestamp(),
                memory.emotional_valence,
                entities_json,
                connections_json,
                memory.id,
            ],
        )?;

        // Update entity index (delete old, insert new)
        self.conn.execute(
            "DELETE FROM entity_index WHERE memory_id = ?1",
            params![memory.id],
        )?;

        for entity in &memory.entities {
            self.conn.execute(
                "INSERT OR IGNORE INTO entity_index (entity, memory_id) VALUES (?1, ?2)",
                params![entity, memory.id],
            )?;
        }

        Ok(())
    }

    /// Convert database row to Memory
    fn row_to_memory(row: &Row) -> rusqlite::Result<Memory> {
        let id: String = row.get(0)?;
        let content: String = row.get(1)?;
        let timestamp_secs: i64 = row.get(2)?;
        let memory_type_str: String = row.get(3)?;
        let emotional_valence: f32 = row.get(4)?;
        let entities_json: String = row.get(5)?;
        let connections_json: String = row.get(6)?;

        let timestamp = DateTime::from_timestamp(timestamp_secs, 0)
            .unwrap_or_else(|| Utc::now());

        let memory_type = match memory_type_str.as_str() {
            "Interaction" => MemoryType::Interaction,
            "Reflection" => MemoryType::Reflection,
            "Curiosity" => MemoryType::Curiosity,
            "EmotionalState" => MemoryType::EmotionalState,
            "WisdomTransformation" => MemoryType::WisdomTransformation,
            "ExistentialReflection" => MemoryType::ExistentialReflection,
            _ => MemoryType::Interaction,
        };

        let entities: Vec<String> = serde_json::from_str(&entities_json)
            .unwrap_or_default();
        let connections: Vec<String> = serde_json::from_str(&connections_json)
            .unwrap_or_default();

        Ok(Memory {
            id,
            content,
            timestamp,
            entities,
            connections,
            memory_type,
            emotional_valence,
            source: MemorySource::DirectExperience, // Default to direct experience
            confidence: 1.0, // Default confidence
        })
    }
}

/// Archive index database
pub struct ArchiveIndexDb {
    conn: Connection,
}

impl ArchiveIndexDb {
    /// Create or open archive index database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Initialize archive index schema
    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS archive_metadata (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                entities TEXT NOT NULL,
                emotional_valence REAL,
                memory_type TEXT,
                content_preview TEXT,
                connections TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_archive_timestamp ON archive_metadata(timestamp DESC)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_archive_entities ON archive_metadata(entities)",
            [],
        )?;

        Ok(())
    }

    /// Add archived memory metadata
    pub fn add_archived(&self, memory: &Memory, file_path: &str) -> Result<()> {
        let entities_json = serde_json::to_string(&memory.entities)?;
        let connections_json = serde_json::to_string(&memory.connections)?;
        let content_preview = memory.content.chars().take(200).collect::<String>();

        self.conn.execute(
            "INSERT OR REPLACE INTO archive_metadata 
             (id, file_path, timestamp, entities, emotional_valence, memory_type, content_preview, connections)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                memory.id,
                file_path,
                memory.timestamp.timestamp(),
                entities_json,
                memory.emotional_valence,
                format!("{:?}", memory.memory_type),
                content_preview,
                connections_json,
            ],
        )?;

        Ok(())
    }

    /// Find relevant archive files by entities
    pub fn find_by_entities(&self, entities: &[String], limit: usize) -> Result<Vec<String>> {
        if entities.is_empty() {
            return Ok(Vec::new());
        }

        // Search for archives containing any of the entities
        let mut file_paths = Vec::new();
        for entity in entities {
            let mut stmt = self.conn.prepare(
                "SELECT DISTINCT file_path
                 FROM archive_metadata
                 WHERE entities LIKE ?1
                 ORDER BY timestamp DESC
                 LIMIT ?2",
            )?;

            let pattern = format!("%\"{}%", entity);
            let paths: Vec<String> = stmt.query_map(params![pattern, limit], |row| row.get(0))?
                .collect::<Result<Vec<_>, _>>()?;

            file_paths.extend(paths);
        }

        // Deduplicate
        file_paths.sort();
        file_paths.dedup();
        file_paths.truncate(limit);

        Ok(file_paths)
    }
}

